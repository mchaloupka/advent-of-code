#load "../util.fsx"
open Util

open System
open System.IO

type MapPoint =
| E
| MU
| MD
| SV
| SH

type WholeMap = MapPoint array array

type MoveDirection =
| L
| R
| U
| D

let printEnergized energized graph =
    printMatrix (fun (ri, ci) ->
        if energized |> Set.contains (ri, ci) then
            '#'
        else
            '.'
    ) graph

let getEnergized start =
    let rec iter visited toVisit (graph: WholeMap) =
        // printfn ""
        // graph |> printEnergized (visited |> Set.map fst)
        
        match toVisit with
        | ((r, c), d)::rest ->
            if visited |> Set.contains ((r, c), d) then
                graph |> iter visited rest
            elif r < 0 || r >= (graph |> Array.length) then
                graph |> iter visited rest
            elif c < 0 || c >= (graph.[r] |> Array.length) then
                graph |> iter visited rest
            else
                let newVisited = visited |> Set.add ((r, c), d)
                
                let up = ((r-1, c), U)
                let down = ((r+1, c), D)
                let left = ((r, c-1), L)
                let right = ((r, c+1), R)

                match graph.[r].[c], d with
                | E, L -> graph |> iter newVisited (left::rest)
                | E, R -> graph |> iter newVisited (right::rest)
                | E, U -> graph |> iter newVisited (up::rest)
                | E, D -> graph |> iter newVisited (down::rest)
                | MU, L -> graph |> iter newVisited (down::rest)
                | MU, R -> graph |> iter newVisited (up::rest)
                | MU, U -> graph |> iter newVisited (right::rest)
                | MU, D -> graph |> iter newVisited (left::rest)
                | MD, L -> graph |> iter newVisited (up::rest)
                | MD, R -> graph |> iter newVisited (down::rest)
                | MD, U -> graph |> iter newVisited (left::rest)
                | MD, D -> graph |> iter newVisited (right::rest)
                | SV, L
                | SV, R ->
                    graph |> iter newVisited (up::down::rest)
                | SV, U -> graph |> iter newVisited (up::rest)
                | SV, D -> graph |> iter newVisited (down::rest)
                | SH, U
                | SH, D ->
                    graph |> iter newVisited (left::right::rest)
                | SH, L -> graph |> iter newVisited (left::rest)
                | SH, R -> graph |> iter newVisited (right::rest)
        | [] ->
            visited
            |> Set.map fst

    iter Set.empty [start]

let input =
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> parseIntoMatrix (function
        | '.' -> E
        | '/' -> MU
        | '\\' -> MD
        | '|' -> SV
        | '-' -> SH
        | c -> failwithf "Unexpected character '%c'" c
    )

input
|> getEnergized ((0,0), R)
|> Set.count
|> printfn "Part 1: %A"

let allStarts =
    [| for i in 0..(input |> Array.length) - 1 do 
        yield ((i, 0), R)
        yield ((i, (input.[i] |> Array.length) - 1), L)
    |]
    |> Array.append [|
        for i in 0..(input.[0] |> Array.length) - 1 do
        yield ((0, i), D)
        yield (((input |> Array.length) - 1, i), U)
    |]

allStarts
|> Array.Parallel.map (fun start -> input |> getEnergized start |> Set.count)
|> Array.max
|> printfn "Part 2: %A"