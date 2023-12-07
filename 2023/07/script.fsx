#load "../util.fsx"
open Util

open System
open System.IO


let cardValue allowJoker = function
    | c when c |> Char.IsDigit -> c |> Char.GetNumericValue |> int
    | 'T' -> 10
    | 'J' -> if allowJoker then 1 else 11
    | 'Q' -> 12
    | 'K' -> 13
    | 'A' -> 14
    | c -> failwithf "Unknown card value for: %A" c

let getHandPower allowJokers (hand: char seq) =
    let cardCounts = 
        hand
        |> Seq.fold (fun counts card ->
            counts
            |> Map.change card (fun x -> x |> Option.map (fun y -> y + 1) |> Option.defaultValue 1 |> Some)
                
        ) Map.empty

    let jCount = cardCounts |> Map.tryFind 'J' |> Option.defaultValue 0

    let findCardGroups count =
        cardCounts
        |> Map.toList
        |> List.choose (fun (card, found) ->
            if card = 'J' && found = count then
                (card, found, found) |> Some
            elif card = 'J' then
                None
            else
                if found = count then
                    (card, found, 0) |> Some
                elif found > count then
                    None
                else
                    if found + jCount >= count && allowJokers then
                        (card, count, count - found) |> Some
                    else
                        None
        )
    
    let canFindCardGroups =
        let rec canFindCardGroupsFrom usedCards remainingJ groupSizes =
            match groupSizes with
            | [] -> true
            | x :: r ->
                let minimumJ =
                    findCardGroups x
                    |> List.filter (fun (card, _, _) -> not (usedCards |> Set.contains card))
                    |> List.sortBy (fun (_, _, usedJ) -> usedJ)
                    |> List.tryHead

                match minimumJ with
                | Some (card, _, usedJ) when usedJ <= remainingJ -> canFindCardGroupsFrom (usedCards |> Set.add card) (remainingJ - usedJ) r
                | _ -> false

        canFindCardGroupsFrom Set.empty jCount

    [
        0, true
        1, canFindCardGroups [ 2 ]
        2, canFindCardGroups [ 2; 2 ]
        3, canFindCardGroups [ 3 ]
        4, canFindCardGroups [ 3; 2 ]
        5, canFindCardGroups [ 4 ]
        6, canFindCardGroups [ 5 ]
    ]
    |> List.choose (fun (label, exists) ->
        if exists then
            label |> Some
        else
            None
    )
    |> List.max

[
    1, false
    2, true
]
|> List.iter (fun (part, allowJoker) ->
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> Seq.map (splitIntoTwo ' ')
    |> Seq.map (fun (hand, bid) -> hand, bid |> int64, hand |> getHandPower allowJoker)
    |> Seq.toList
    |> List.sortWith (fun (hand1, _, power1) (hand2, _, power2) ->
        if power1 = power2 then
            let rec handCompare h1 h2 =
                match h1, h2 with
                | c1::r1, c2::r2 ->
                    let p1 = c1 |> cardValue allowJoker
                    let p2 = c2 |> cardValue allowJoker
                    if p1 = p2 then
                        handCompare r1 r2
                    else
                        p1 - p2
                | [], [] -> 0
                | _, _ -> failwithf "Incomparable hands %A %A" hand1 hand2

            handCompare (hand1 |> Seq.toList) (hand2 |> Seq.toList)
        else
            power1 - power2
    )
    |> List.mapi (fun i (_, bid, _) -> (int64 i + 1L) * bid)
    |> List.sum
    |> printfn "Part %d: %A" part
)
