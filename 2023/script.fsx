#load "../util.fsx"
open Util

open System
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
        let (rr, cc) = getOrigIndices x

        match input.[rr].[cc] with
        | R -> false
        | _ -> true
    )

let getFromCache ((r, c), stepCount) reachableCache =
    let (rr, cc) = getOrigIndices (r, c)

    reachableCache
    |> Map.tryFind stepCount
    |> Option.defaultValue Map.empty
    |> Map.tryFind (rr, cc)
    |> Option.map (
        Set.map (fun (x, y) -> x + r - rr, y + c - cc)
    )

let isInCache (x, stepCount) reachableCache =
    let t = getOrigIndices x
    
    reachableCache
    |> Map.tryFind stepCount
    |> Option.defaultValue Map.empty
    |> Map.containsKey t

let addToCache ((r, c), stepCount) reachables reachableCache =
    let (rr, cc) = getOrigIndices (r, c)

    reachableCache
    |> Map.change stepCount (fun stepMap ->
        stepMap
        |> Option.defaultValue Map.empty
        |> Map.add (rr, cc) (reachables |> Set.map (fun (x, y) -> x + rr - r, y + cc - c))
        |> Some
    )

let getTransitiveReachable reachableCache nextSteps reachables =
    reachables
    |> Set.fold (fun (transitives, missing) item ->
        match reachableCache |> getFromCache (item, nextSteps) with
        | None -> Set.empty, missing |> Set.add item
        | Some _ when missing |> Set.isEmpty |> not -> Set.empty, missing
        | Some tt -> transitives |> Set.union tt, missing
    ) (Set.empty, Set.empty)
    |> fun (transitives, missing) -> transitives, missing |> Set.map (fun x -> x, nextSteps) |> Set.toList

let precomputeReachables stepCount =
    let doubleStepsNeeded = ((stepCount - 1 |> double |> log) / log 2.0) |> floor
    let desiredCount = 2.0 ** doubleStepsNeeded |> int

    let rec iter reachableCache curCount =
        if curCount > desiredCount then
            printfn "Done precomputing"
            reachableCache
        elif curCount = 1 then
            printfn "Precomputing layer: 1"

            let newCache =
                input
                |> allIndices
                |> Seq.toArray
                |> Array.Parallel.map (fun x ->
                    x, reachableInOneStep x
                )
                |> Map.ofArray
            iter (reachableCache |> Map.add curCount newCache) (curCount * 2)
        else
            printfn "Precomputing layer: %d" curCount

            let prevCount = curCount / 2

            let newCache =
                input
                |> allIndices
                |> Seq.toArray
                |> Array.Parallel.map (fun x ->
                    let reachables = reachableCache |> getFromCache (x, prevCount) |> Option.get
                    let transitiveReachable =
                        reachables
                        |> getTransitiveReachable reachableCache prevCount
                        |> fst

                    x, transitiveReachable
                )
                |> Map.ofArray
            
            iter (reachableCache |> Map.add curCount newCache) (curCount * 2)

    iter Map.empty 1

let reachableInSteps stepCount =
    let rec iter reachableCache toProcess =
        match toProcess with
        | x::rest when reachableCache |> isInCache x -> iter reachableCache rest
        | [] -> reachableCache |> getFromCache (startIndices, stepCount) |> Option.get
        | (x, 1)::rest ->
            iter (reachableCache |> addToCache (x, 1) (reachableInOneStep x)) rest
        | (x, s)::rest ->
            let doubleStepsNeeded = ((s - 1 |> double |> log) / log 2.0) |> floor
            let nextSubStep = 2.0 ** doubleStepsNeeded |> int
            let otherNextSubStep = s - nextSubStep

            printfn "Handling %A" (x, s)

            match reachableCache |> getFromCache (x, otherNextSubStep) with
            | None -> iter reachableCache ((x, otherNextSubStep)::(x,s)::rest)
            | Some reachables ->
                let (transitiveReachable, missing) =
                    reachables
                    |> getTransitiveReachable reachableCache nextSubStep

                if missing |> List.isEmpty then
                    iter (reachableCache |> addToCache (x, s) transitiveReachable) rest
                else
                    iter reachableCache (missing @ (x,s)::rest)
    
    iter (precomputeReachables stepCount) [ (startIndices, stepCount ) ]

reachableInSteps 26501365
|> Set.count
|> printfn "%A"
