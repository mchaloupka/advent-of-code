#load "../util.fsx"
open Util

open System.IO

type Card = { Id: int; WinningNumbers: Set<int>; MyNumbers: Set<int> }

let parseLine (line: string) =
    let split = line.Split([|':';'|'|])
    let cardId = split.[0].Trim().Split([|' '|], System.StringSplitOptions.RemoveEmptyEntries).[1] |> int

    let readNumbers (linePart: string) =
        linePart.Trim().Split([|' '|], System.StringSplitOptions.RemoveEmptyEntries)
        |> Array.map (fun x -> x.Trim() |> int)
        |> Set.ofArray

    {
        Id = cardId
        WinningNumbers = split.[1] |> readNumbers
        MyNumbers = split.[2] |> readNumbers
    }

let cards = 
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> Seq.map parseLine

let part1 =
    cards
    |> Seq.map (fun card -> 
        Set.intersect card.WinningNumbers card.MyNumbers
        |> Set.toList
        |> List.fold (fun agg _ -> if agg = 0 then 1 else agg * 2) 0
    )
    |> Seq.sum

printfn "%A" part1

let part2 =
    cards
    |> Seq.fold (fun (count, extras) card ->
        let addCardMultiplier multiplier cardId (updatedExtras: Map<int, int>) =
            updatedExtras
            |> Map.change cardId (fun x -> x |> Option.map (fun t -> t + multiplier) |> Option.defaultValue (multiplier + 1) |> Some)

        let matches = Set.intersect card.WinningNumbers card.MyNumbers |> Set.count

        let cardMultiplier =
            extras
            |> Map.tryFind card.Id
            |> Option.defaultValue 1

        count + cardMultiplier, [1..matches] |> List.fold (fun agg offset -> agg |> addCardMultiplier cardMultiplier (card.Id + offset)) extras
    ) (0, Map.empty)
    |> fst

printfn "%A" part2
