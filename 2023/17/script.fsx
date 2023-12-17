#r "nuget: FSharpx.Collections, 3.1.0"

#load "../util.fsx"
open Util

open System
open System.IO

open FSharpx.Collections

type WholeMap = int array array
type Direction = | L | R | U | D
type Point = int * int

// Cost is first to ensure that the order in heap is ordered by Cost first
type MovementState =
    { Cost: int; Direction: Direction; Point: Point; InSameDirection: int }

let input =
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> parseIntoMatrix (function
        | c when c |> Char.IsDigit -> c |> Char.GetNumericValue |> int
        | c -> failwithf "Unexpected character '%c'" c
    )

let getCost (ri, ci) (graph: WholeMap) =
    graph.[ri].[ci]

let findCheapestPath nextMovements isFinished (input: WholeMap) =
    let rec iter visited toVisit =
        if toVisit |> Heap.isEmpty |> not then
            let getVisitedKey movementState =
                (movementState.Point, movementState.Direction, movementState.InSameDirection)

            let movementState = toVisit |> Heap.head

            if input |> isFinished movementState then
                movementState.Cost
            else
                let nextMovements =
                    if visited |> Set.contains (getVisitedKey movementState) then
                        []
                    else
                        input |> nextMovements movementState
                
                let nextToVisit =
                    nextMovements
                    |> List.fold (fun agg x ->
                        agg |> Heap.insert x
                    ) (toVisit |> Heap.tail)

                let nextVisited = visited |> Set.add (getVisitedKey movementState)

                iter nextVisited nextToVisit
        else
            failwithf "Finish not found"

    let initialState = {
        Direction = R
        Point = 0, 0
        InSameDirection = 0
        Cost = 0
    }

    let toVisit =
        Heap.empty false
        |> Heap.insert initialState
        |> Heap.insert { initialState with Direction = D }
    
    iter Set.empty toVisit

let nextMovements movementState (graph: WholeMap) =
    let (ri, ci) = movementState.Point

    let left = (ri, ci - 1), L
    let right = (ri, ci + 1), R
    let up = (ri - 1, ci), U
    let down = (ri + 1, ci), D

    match movementState.Direction with
    | L -> [ (left, movementState.InSameDirection + 1); (up, 0); (down, 0) ]
    | R -> [ (right, movementState.InSameDirection + 1); (up, 0); (down, 0) ]
    | U -> [ (up, movementState.InSameDirection + 1); (left, 0); (right, 0) ]
    | D -> [ (down, movementState.InSameDirection + 1); (left, 0); (right, 0) ]
    |> List.filter (fun (((ri, ci), _), inDirection) -> 
        ri >= 0 && ri < (graph |> Array.length) // Can't go outside
            && ci >= 0 && ci < (graph.[0] |> Array.length)
            && inDirection < 3 // Can't go 4-times in the same direction
    )
    |> List.map (fun ((nextPoint, direction), inDirection) ->
        { 
            Point = nextPoint
            Direction = direction
            InSameDirection = inDirection
            Cost = movementState.Cost + (graph |> getCost nextPoint)
        }
    )

let isFinished movementState input =
    let (ri, ci) = movementState.Point
    ri = (input |> Array.length) - 1 && ci = (input.[0] |> Array.length) - 1

input
|> findCheapestPath nextMovements isFinished
|> printfn "Part 1: %A"

let nextMovementsUltra movementState (graph: WholeMap) =
    let (ri, ci) = movementState.Point

    let left = (ri, ci - 1), L
    let right = (ri, ci + 1), R
    let up = (ri - 1, ci), U
    let down = (ri + 1, ci), D

    match movementState.Direction with
    | L -> [ (left, movementState.InSameDirection + 1); (up, 1); (down, 1) ]
    | R -> [ (right, movementState.InSameDirection + 1); (up, 1); (down, 1) ]
    | U -> [ (up, movementState.InSameDirection + 1); (left, 1); (right, 1) ]
    | D -> [ (down, movementState.InSameDirection + 1); (left, 1); (right, 1) ]
    |> List.filter (fun (((ri, ci), _), inDirection) -> 
        ri >= 0 && ri < (graph |> Array.length) // Can't go outside
            && ci >= 0 && ci < (graph.[0] |> Array.length)
            && inDirection < 11 // Can't go 11-times in the same direction
            && (movementState.InSameDirection >= 4 || inDirection > 1 || movementState.InSameDirection = 0) // Cannot turn if it did not go 4-times in the same direction
    )
    |> List.map (fun ((nextPoint, direction), inDirection) ->
        { 
            Point = nextPoint
            Direction = direction
            InSameDirection = inDirection
            Cost = movementState.Cost + (graph |> getCost nextPoint)
        }
    )

let isFinishedUltra movementState input =
    (input |> isFinished movementState) && movementState.InSameDirection >= 4

input
|> findCheapestPath nextMovementsUltra isFinishedUltra
|> printfn "Part 1: %A"