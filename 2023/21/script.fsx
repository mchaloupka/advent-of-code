#load "../util.fsx"
open Util

open System.IO

type Point = | R | G | S

let input =
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> parseIntoMatrix (function
        | 'S' -> S
        | '.' -> G
        | '#' -> R
        | c -> failwithf "Unexpected character '%c'" c
    )

let startIndices =
    input
    |> allIndices
    |> Seq.find (fun (r, c) -> input.[r].[c] = S)

let getOrigIndices (r, c) =
    euclideanMod r (input |> Array.length), euclideanMod c (input.[0] |> Array.length)

let reachableInOneStep (r, c) =
    Set [
        r - 1, c
        r + 1, c
        r, c - 1
        r, c + 1
    ]
    |> Set.filter (fun x ->
        let (r, c) = x |> getOrigIndices
        match input.[r].[c] with
        | R -> false
        | _ -> true
    )

let countReachable (start, stepCount) =
    let rec iter agg alreadySeen toProcess =
        match toProcess with
        | [] -> agg
        | (_, rs)::rest when rs < 0 -> iter agg alreadySeen rest
        | (x, rs)::rest ->
            let newAgg =
                if rs % 2 = 0 then
                    agg |> Set.add x
                else
                    agg

            let nextSteps =
                reachableInOneStep x
                |> Set.filter (fun y -> 
                    alreadySeen |> Set.contains y |> not
                )

            let newAlreadySeen = alreadySeen |> Set.union nextSteps
            let newToProcess =
                nextSteps
                |> Set.map (fun x -> x, rs - 1)
                |> Set.toList
                |> fun x -> rest @ x

            iter newAgg newAlreadySeen newToProcess

    iter Set.empty Set.empty [start, stepCount]

let part1Input = startIndices, 64

let getCount = Set.count >> bigint

countReachable part1Input
|> getCount
|> printfn "Part 1: %A"

// Part 2

// Grid is a square 131 x 131
// We start in middle
// There is a clear path to sides
// Let n = 0 be side / 2 steps
//     n = 1 be side + side / 2 steps
//     n = 2 be 2 * side + side / 2 steps

let stepCount = 26501365
let side = input |> Array.length
let half = startIndices |> fst // = side / 2

let f0 = countReachable (startIndices, half) |> getCount
let f1 = countReachable (startIndices, half + side) |> getCount
let f2 = countReachable (startIndices, half + 2 * side) |> getCount

// I am looking for quadratic function a * x^2 + b * x + c = count for the points above
// Gauss elimination leads to:
let c = f0
let a = (f2 - (bigint 2) * f1 + f0) / (bigint 2)
let b = f1 - f0 - a

// That gives me the following function
let quadratic n = a * n * n + b * n + c

let n = (stepCount - half) / side |> bigint

quadratic n
|> printfn "Part 2: %A"
