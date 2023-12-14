#load "../util.fsx"
open Util

open System
open System.IO

type MapPoint =
| R
| C
| E

type Direction =
| North
| West
| South
| East

type MirrorMapData = Map<int, Map<int, MapPoint>>

type MirrorMap =  { data: MirrorMapData; direction: Direction }

let transpose (map: MirrorMap) =
    match map.direction with
    | North -> { map with direction = West }
    | West -> { map with direction = South }
    | South -> { map with direction = East }
    | East -> { map with direction = North }

let updateAtIndex ri ci newValue (data: MirrorMapData) =
    let oldRow = data |> Map.tryFind ri |> Option.defaultValue Map.empty
    let newRow = oldRow |> Map.add ci newValue
    data |> Map.add ri newRow

let mapHeight (map: MirrorMap) =
    match map.direction with
    | North | South -> map.data |> Map.count
    | West | East -> map.data |> Map.find 0 |> Map.count

let mapWidth (map: MirrorMap) =
    match map.direction with
    | North | South -> map.data |> Map.find 0 |> Map.count
    | West | East -> map.data |> Map.count

let getIndex ri ci (map: MirrorMap) =
    match map.direction with
    | North -> ri, ci
    | South -> (map.data |> Map.count) - ri - 1, ci
    | West -> ci, ri
    | East -> ci, (map.data |> Map.find 0 |> Map.count) - ri - 1

let getValue ri ci (map: MirrorMap) =
    let (tri, tci) = map |> getIndex ri ci
    map.data |> Map.find tri |> Map.find tci

let updateValue ri ci newValue (map: MirrorMap) =
    let (tri, tci) = map |> getIndex ri ci
    { map with data = map.data |> updateAtIndex tri tci newValue }

let parseMap (input: MapPoint array array) =
    let data = 
        input
        |> allIndices
        |> Seq.fold (fun agg (ri, ci) ->
            agg |> updateAtIndex ri ci (input.[ri].[ci])
        ) Map.empty
    { data = data; direction = North }

let printMap (map:MirrorMap) = 
    printfn ""
    printfn "Next direction: %A" map.direction

    map.data
    |> Map.toSeq
    |> Seq.sortBy (fun (ri, _) -> ri)
    |> Seq.iter (fun (_, row) ->
        row
        |> Map.toSeq
        |> Seq.sortBy (fun (ci, _) -> ci)
        |> Seq.map (fun (_, p) ->
            match p with
            | R -> 'O'
            | C -> '#'
            | E -> '.'
        )
        |> Seq.iter (printf "%c")

        printfn ""
    )

let moveRocksUp =
    let rec place canMoveRockTo ri ci (map: MirrorMap) =
        if ci >= (map |> mapWidth) then
            if (ri + 1) = (map |> mapHeight) then
                map
            else
                place canMoveRockTo (ri + 1) 0 map
        else
            match map |> getValue ri ci with
            | R ->
                let moveTo = canMoveRockTo |> Map.tryFind ci |> Option.defaultValue 0
                let newMovementMap = canMoveRockTo |> Map.add ci (moveTo + 1)

                let updatedMap =
                    map
                    |> updateValue ri ci E
                    |> updateValue moveTo ci R

                place newMovementMap ri (ci + 1) updatedMap
            | C ->
                let newMovementMap = canMoveRockTo |> Map.add ci (ri + 1)
                place newMovementMap ri (ci + 1) map
            | E ->
                place canMoveRockTo ri (ci + 1) map
    place Map.empty 0 0

let calculateLoad (map: MirrorMap) =
    map.data
    |> Map.toSeq
    |> Seq.map (fun (ri, row) ->
        row 
        |> Map.toSeq
        |> Seq.map (fun (_, p) ->
            match p with
            | R -> (map |> mapHeight) - ri
            | _ -> 0
        )
        |> Seq.sum
    )
    |> Seq.sum

Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
|> readLines
|> parseIntoMatrix (function
    | 'O' -> R
    | '#' -> C
    | '.' -> E
    | c -> failwithf "Unexpected character '%c'" c
)
|> parseMap
|> moveRocksUp
|> calculateLoad
|> printfn "Part 1: %A"

let performCycle map =
    map
    |> moveRocksUp
    |> transpose
    |> moveRocksUp
    |> transpose
    |> moveRocksUp
    |> transpose
    |> moveRocksUp
    |> transpose

let performCycles =
    let rec iter prevStates performedCycles count map =
        if performedCycles = count then
            map
        else
            let afterCycle =
                map
                |> performCycle

            let cycleCount = performedCycles + 1

            match prevStates |> Map.tryFind afterCycle.data with
            | Some x ->
                let periodLength = cycleCount - x
                let remainingIterations = count - cycleCount
                let remainingIterationsAfterRepeatingPeriod = remainingIterations % periodLength

                iter Map.empty 0 remainingIterationsAfterRepeatingPeriod afterCycle
            | None ->
                let updatedStates = prevStates |> Map.add afterCycle.data cycleCount
                iter updatedStates cycleCount count afterCycle
    
    iter Map.empty 0

Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
|> readLines
|> parseIntoMatrix (function
    | 'O' -> R
    | '#' -> C
    | '.' -> E
    | c -> failwithf "Unexpected character '%c'" c
)
|> parseMap
|> performCycles 1000000000
|> calculateLoad
|> printfn "Part 2: %A"
