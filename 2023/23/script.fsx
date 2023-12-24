#load "../util.fsx"
open Util

open System.IO

type Field =
    | F
    | P
    | SU
    | SD
    | SL
    | SR

let input =
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> parseIntoMatrix (function
        | '#' -> F
        | '.' -> P
        | '>' -> SR
        | '^' -> SU
        | '<' -> SL
        | 'v' -> SD
        | c -> failwithf "Unexpected character %A" c
    )

let getOnlyPath rowIx =
    let colIx =
        [ for i in 0..(input.[rowIx] |> Array.length) - 1 do
            if input.[rowIx].[i] = P then
                i |> Some
            else
                None
        ]
        |> List.choose id
        |> List.head
    rowIx, colIx

let startPoint = getOnlyPath 0
let endPoint = (input |> Array.length) - 1 |> getOnlyPath

type Edge = { To: int * int; Cost: int }

let getGraph neighbohours =
    input
    |> allIndices
    |> Seq.filter (fun (r, c) -> input.[r].[c] = F |> not)
    |> Seq.fold (fun result point ->
        let allEdges = 
            point 
            |> neighbohours
            |> List.map (fun x -> { To = x; Cost = 1 })
        result |> Map.add point allEdges
    ) Map.empty

let reduceGraph graph =
    let replacePointInEdges from oldTo newTo newCost graph =
        graph
        |> Map.change from (Option.get >> List.map (fun edge ->
            if edge.To = oldTo then
                { To = newTo; Cost = newCost }
            else
                edge
        ) >> Some)


    graph
    |> Map.toSeq
    |> Seq.filter (fun (_, edges) -> edges |> List.length = 2)
    |> Seq.fold (fun result (point, _) ->
        let edges = result |> Map.find point
        let (point1, point2, totalCost) =
            match edges with
            | [ x; y] -> x.To, y.To, x.Cost + y.Cost
            | _ -> failwith "Should not happen - just two edges are expected"
        
        result
        |> Map.remove point
        |> replacePointInEdges point1 point point2 totalCost
        |> replacePointInEdges point2 point point1 totalCost
    ) graph

let longestPath fromPoint toPoint graph =
    let rec iter longestPath toProcess =
        match toProcess with
        | [] -> longestPath
        | (point, _, length)::rest when point = toPoint ->
            if longestPath > length then
                iter longestPath rest
            else
                iter length rest
        | (point, visited, length)::rest ->
            let newVisited = visited |> Set.add point

            let nextToVisit =
                graph 
                |> Map.find point
                |> List.filter (fun x -> visited |> Set.contains x.To |> not)
                |> List.map (fun x -> x.To, newVisited, length + x.Cost)
            
            iter longestPath (nextToVisit @ rest)

    (fromPoint, Set.empty, 0)
    |> List.singleton
    |> iter 0

let neighbohours (r, c) =
    let left = (r, c - 1)
    let up = (r - 1, c)
    let down = (r + 1, c)
    let right = (r, c + 1)

    match input.[r].[c] with
    | P -> [ left; up; down; right ]
    | F -> failwith "Should never been asked"
    | SR -> [ right ]
    | SL -> [ left ]
    | SU -> [ up ]
    | SD -> [ down ]
    |> List.filter (fun (rr, cc) ->
        rr >= 0 && rr < (input |> Array.length) && cc >= 0 && cc < (input.[0] |> Array.length)
    )
    |> List.filter (fun (rr, cc) ->
        match input.[rr].[cc] with
        | F -> false
        | _ -> true
    )

neighbohours
|> getGraph
|> reduceGraph
|> longestPath startPoint endPoint
|> printfn "Part 1: %A"

let nonSteepNeighbohours (r, c) =
    let left = (r, c - 1)
    let up = (r - 1, c)
    let down = (r + 1, c)
    let right = (r, c + 1)

    match input.[r].[c] with
    | F -> failwith "Should never been asked"
    | _ -> [ left; up; down; right ]
    |> List.filter (fun (rr, cc) ->
        rr >= 0 && rr < (input |> Array.length) && cc >= 0 && cc < (input.[0] |> Array.length)
    )
    |> List.filter (fun (rr, cc) ->
        match input.[rr].[cc] with
        | F -> false
        | _ -> true
    )

nonSteepNeighbohours
|> getGraph
|> reduceGraph
|> longestPath startPoint endPoint
|> printfn "Part 2: %A"