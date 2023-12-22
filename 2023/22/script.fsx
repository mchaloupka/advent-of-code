#load "../util.fsx"
open Util

open System.IO

type BrickPoint = { X: int; Y: int; Z: int }
type Brick = { Ix: int; From: BrickPoint; To: BrickPoint }

let parseBrickPoint coords =
    match coords |> splitBy ',' with
    | [x;y;z] ->
        {
            X = x |> int
            Y = y |> int
            Z = z |> int
        }
    | _ -> failwithf "Invalid brick coord %A" coords


let parseBrick ix line =
    let (fromRaw, toRaw) = line |> splitIntoTwo '~'
    {
        Ix = ix
        From = fromRaw |> parseBrickPoint
        To = toRaw |> parseBrickPoint
    }

let input =
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> Seq.mapi parseBrick
    |> Seq.toList

let settledBricksWithSupports =
    let rec iter processed toProcess =
        match toProcess with
        | [] -> processed
        | b::rest ->
            let overlapping =
                processed
                |> Map.keys
                |> Seq.filter (fun bb ->
                    b.From.X <= bb.To.X && b.To.X >= bb.From.X && b.From.Y <= bb.To.Y && b.To.Y >= bb.From.Y
                )

            let (support, fallsTo) =
                if overlapping |> Seq.isEmpty then
                    Set.empty, 0
                else
                    let highestOverlap =
                        overlapping
                        |> Seq.sortByDescending _.To.Z
                        |> Seq.map _.To.Z
                        |> Seq.head

                    let supports =
                        overlapping
                        |> Seq.filter (fun b -> b.To.Z = highestOverlap)
                        |> Set.ofSeq

                    supports, highestOverlap + 1

            let settledBrick =
                { b with 
                    From = { b.From with Z = fallsTo }
                    To = { b.To with Z = b.To.Z - b.From.Z + fallsTo }
                }

            iter (processed |> Map.add settledBrick support) rest

    input
    |> List.sortBy _.From.Z
    |> iter Map.empty

let bricksThatCannotBeRemoved =
    settledBricksWithSupports
    |> Map.filter (fun _ x -> x |> Set.count = 1)
    |> Map.values
    |> Set.ofSeq

(input |> List.length) - (bricksThatCannotBeRemoved |> Set.count)
|> printfn "Part 1: %A"

let bricksSupporting =
    settledBricksWithSupports
    |> Seq.collect (fun x -> x.Value |> Set.map (fun y -> y, x.Key))
    |> Seq.groupBy fst
    |> Map.ofSeq
    |> Map.map (fun _ x -> x |> Seq.map snd |> Set.ofSeq)

let bricksThatFallInChain removedBrick =
    let rec iter falling toProcess =
        match toProcess with
        | [] -> falling
        | x::rest ->

            let allFalling = falling |> Set.add x

            let supporting = 
                bricksSupporting 
                |> Map.tryFind x
                |> Option.defaultValue Set.empty

            let nextFallingBricks = 
                supporting
                |> Set.filter (fun y ->
                    let supportedBy = settledBricksWithSupports |> Map.find y
                    Set.isSubset supportedBy allFalling
                )
                |> Set.toList

            iter allFalling (rest @ nextFallingBricks)

    iter Set.empty [removedBrick]
    |> Set.remove removedBrick

settledBricksWithSupports
|> Map.keys
|> Seq.toList
|> List.map (bricksThatFallInChain >> Set.count)
|> List.sum
|> printfn "Part 2: %A"
