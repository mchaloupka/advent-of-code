#load "../util.fsx"
open Util

open System
open System.IO

let getHash =
    Seq.fold (fun agg (nextChar: char) ->
        let curValue = nextChar |> int
        ((agg + curValue) * 17) % 256
    ) 0

Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
|> readLines
|> (extractFirstLine >> fst)
|> splitBy ','
|> List.map (getHash >> int64)
|> List.sum
|> printfn "Part 1: %A"

Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
|> readLines
|> (extractFirstLine >> fst)
|> splitBy ','
|> List.fold (fun (agg: Map<int, (string * int) list>) entry ->
    if entry.EndsWith "-" then
        let toRemove = entry.Substring(0, entry.Length - 1)
        let updatedEntries = 
            agg 
            |> Map.tryFind (toRemove |> getHash) 
            |> Option.defaultValue List.empty
            |> List.filter (fun (key, _) -> key = toRemove |> not)
        agg |> Map.add (toRemove |> getHash) updatedEntries
    elif entry.Contains '=' then
        let (name, valRaw) = entry |> splitIntoTwo '='
        let value = valRaw |> int
        
        let existing = 
            agg 
            |> Map.tryFind (name |> getHash) 
            |> Option.defaultValue List.empty

        let (found, updated) =
            existing
            |> List.fold (fun (found, result) (k, v) ->
                if k = name then
                    (true, (name, value)::result)
                else
                    (found, (k, v)::result)
            ) (false, List.empty)

        let updatedEntries =
            if found then
                updated
            else
                (name, value)::updated
            |> List.rev
            
        agg |> Map.add (name |> getHash) updatedEntries
    else
        failwithf "Unexpected command: %s" entry
) Map.empty
|> Map.toList
|> List.map (fun (boxNumber, lenses) ->
    lenses
    |> List.mapi (fun lensIndex (lens, focalLength) ->
       (boxNumber + 1) * (lensIndex + 1) * focalLength
    )
    |> List.map int64
    |> List.sum
)
|> List.sum
|> printfn "Part 2: %A"