#load "../util.fsx"
open Util

open System
open System.IO

type Direction = | R | D | L | U

let getVertices instructions =
    instructions
    |> Seq.scan (fun (x, y) (direction, length) ->
        match direction with
        | R -> (x, y + length)
        | L -> (x, y - length)
        | D -> (x + length, y)
        | U -> (x - length, y)
    ) (bigint 0, bigint 0)

let getIntegerPointsOnEdgesCount instructions =
    instructions
    |> Seq.map (fun (_, length: bigint) -> length)
    |> Seq.sum

let getPolygonArea instructions =
    // Shoelace algorithm for area
    let rec iter agg points =
        match points with
        | (x1, y1)::(x2, y2)::rest ->
            iter (agg + ((x1 * y2) - (x2 * y1))) ((x2, y2)::rest)
        | _ ->
            (agg / (bigint 2)) |> abs

    // The edges represents whole boxes that should be included.
    // However, ~ half of the points is already included in the area.
    // We need to add also the ones outside of area. For every point inside 
    // there is another one outside. Except for two that are just outside.
    let pointsOnEdgesCount = instructions |> getIntegerPointsOnEdgesCount

    ((pointsOnEdgesCount + (bigint 2)) / (bigint 2)) + (instructions |> getVertices |> Seq.toList |> (iter (bigint 0)))

let parseInputPart1 = function
    | [ dirRaw; lengthRaw; _ ] ->
        let length = lengthRaw |> int |> bigint
        let direction =
            match dirRaw with
            | "R" -> R
            | "D" -> D
            | "L" -> L
            | "U" -> U
            | _ -> failwithf "Unexpected direction %A" dirRaw
        (direction, length)
    | x -> failwithf "Unexpected match %A" x

let parseInputPart2 = function
    | [ _; _; colour: string ] ->
        let distanceRaw = colour[0..colour.Length - 2]
        let direction = 
            match colour[colour.Length - 1..] with
            | "0" -> R
            | "1" -> D
            | "2" -> L
            | "3" -> U
            | _ -> failwithf "Unexpected direction %s" colour

        direction, Convert.ToInt32(distanceRaw, 16) |> bigint
    | x -> failwithf "Unexpected match %A" x

[
    "Part 1", parseInputPart1
    "Part 2", parseInputPart2
]
|> List.iter (fun (part, parse) ->
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> Seq.map (splitByRegexGroups @"(\w+) (\d+) \(#(\w+)\)")
    |> Seq.map parse
    |> getPolygonArea
    |> printfn "%s: %A" part
)
