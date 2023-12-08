#load "../util.fsx"
open Util

open System
open System.IO

type GraphNode = { Left: string; Right: string }

let parseGraph =
    Seq.fold (fun graph line ->
        let (from, left, right) =
            match line |> splitByRegexGroups @"(\w+) = \((\w+), (\w+)\)" with
            | [f; l; t] -> f, l, t
            | x -> failwithf "Invalid input '%s' parsed as %A" line x
        graph |> Map.add from { Left = left; Right = right }
    ) Map.empty

let moveBy graph instruction curNode =
    let moveFun =
        match instruction with
        | 'L' -> fun x -> x.Left
        | 'R' -> fun x -> x.Right
        | x -> failwithf "Unexpected instruction %A" x
    graph |> Map.tryFind curNode |> Option.map moveFun |> Option.defaultWith (fun x -> failwithf "Can't find node '%s' in graph '%A'" curNode graph)

let getPathLengthFromTo startNode endNodes instructions graph =
    let rec getPathLengthRec pathLength remainingInstructions curNode =
        if endNodes |> Set.contains curNode then pathLength
        else
            match remainingInstructions with
            | [] -> getPathLengthRec pathLength instructions curNode
            | x::rest -> curNode |> moveBy graph x |> getPathLengthRec (pathLength + 1L) rest
    
    getPathLengthRec 0 instructions startNode

// Function to calculate the Greatest Common Divisor
let rec gcd a b =
    if b = 0L then a
    else gcd b (a % b)

// Function to calculate the Least Common Multiple
let lcm a b =
    (a * b) / gcd a b

let getPahtLengthPart2 instructions graph =
    let lastChar = fun x -> x |> Seq.rev |> Seq.head
    let nodesWithLastChar c = graph |> Map.keys |> Seq.filter (fun x -> x |> lastChar = c)
    let endNodes = nodesWithLastChar 'Z' |> Set.ofSeq
    let startNodes = nodesWithLastChar 'A' |> List.ofSeq

    startNodes
    |> List.map (fun startNode -> getPathLengthFromTo startNode endNodes instructions graph)
    |> List.reduce lcm

[
    1, getPathLengthFromTo "AAA" ("ZZZ" |> Set.singleton)
    2, getPahtLengthPart2
]
|> List.iter (fun (part, getPathLength) ->
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> extractFirstLine
    |> fun (firstLine, rest) -> firstLine, rest |> extractFirstLine |> snd
    |> fun (instructions, graphStrings) -> instructions, graphStrings |> parseGraph
    |> fun (instructions, graph) -> getPathLength (instructions |> Seq.toList) graph
    |> printfn "Part %d: %A" part
)
