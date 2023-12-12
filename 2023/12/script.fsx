#load "../util.fsx"
open Util

open System
open System.IO

let rec getCombinations resultCache curGroupSize remainingGroups input =
    let cacheKey = (curGroupSize, remainingGroups, input)

    let responseWithUpdatedCache (response: int64) cache =
        (cache |> Map.add cacheKey response), response
    
    match resultCache |> Map.tryFind (curGroupSize, remainingGroups, input) with
    | Some cached -> resultCache |> responseWithUpdatedCache cached
    | None ->
        match input with
        | '#'::rest ->
            match remainingGroups with
            | f::_ when curGroupSize >= f -> resultCache |> responseWithUpdatedCache 0L
            | [] -> resultCache |> responseWithUpdatedCache 0L
            | _ -> getCombinations resultCache (curGroupSize + 1) remainingGroups rest
        | '.'::rest ->
            match remainingGroups with
            | _ when curGroupSize = 0 -> getCombinations resultCache 0 remainingGroups rest
            | f::other when f = curGroupSize -> getCombinations resultCache 0 other rest
            | _ -> resultCache |> responseWithUpdatedCache 0L
        | '?'::rest ->
            let (withSharpResultCache, withSharpValue) = getCombinations resultCache curGroupSize remainingGroups ('#'::rest)
            let (withDotResultCache, withDotValue) = getCombinations withSharpResultCache curGroupSize remainingGroups ('.'::rest)
            withDotResultCache |> responseWithUpdatedCache (withSharpValue + withDotValue)
        | [] ->
            if curGroupSize > 0 then
                getCombinations resultCache curGroupSize remainingGroups ['.']
            elif remainingGroups |> List.isEmpty then
                resultCache |> responseWithUpdatedCache 1L
            else
                resultCache |> responseWithUpdatedCache 0L
        | c::_ -> failwithf "Unexpected input character: '%c'" c

let part1InputPostProcess (recordInput: string) (numbersInput: int list) =
    recordInput, numbersInput

let part2InputPostProcess (recordInput: string) (numbersInput: int list) =
    let numbers = [ for _ in 1..5 do numbersInput ] |> List.collect id
    let record = String.Join('?', [| for _ in 1..5 do recordInput |])
    record, numbers

let processLine inputPostProcess line =
    let (recordInput, numbersRaw) = line |> splitIntoTwo ' '
    let numbersInput = numbersRaw |> splitBy ',' |> List.map int

    let (record, numbers) = inputPostProcess recordInput numbersInput

    record |> List.ofSeq |> getCombinations Map.empty 0 numbers |> snd

[
    "Part 1", part1InputPostProcess
    "Part 2", part2InputPostProcess
]
|> List.iter (fun (part, inputPostProcess) ->
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> Array.ofSeq
    |> Array.Parallel.sumBy (processLine inputPostProcess)
    |> printfn "%s: %A" part
)
