#load "../util.fsx"
open Util

open System
open System.IO

type LavaPoint =
| A
| R

type LavaIsland = LavaPoint array array

let verticalEncode (lavaIsland: LavaIsland) =
    [| for ci in 0..(lavaIsland.[0] |> Array.length) - 1 do 
        [| for ri in 0..(lavaIsland |> Array.length) - 1 do
            lavaIsland.[ri].[ci]
        |]
    |]

let horizontalEncode = id

let findReflectionsPart1 encode (lavaIsland: LavaIsland) =
    let encoded = lavaIsland |> encode

    seq { for mix in 1..(encoded |> Array.length) - 1 do mix }
    |> Seq.filter (fun mix ->
        seq { for i in 0..(mix - 1) do i }
        |> Seq.forall (fun ix ->
            let mirroredIx = (mix - ix) + mix - 1

            if mirroredIx >= (encoded |> Array.length) then
                true
            else
                encoded.[ix] = encoded.[mirroredIx]
        )
    )

let calculateReflections findReflections lavaIsland =
    let verticalReflection = lavaIsland |> findReflections verticalEncode |> Seq.tryHead |> Option.defaultValue 0
    let horizontalReflection = lavaIsland |> findReflections horizontalEncode |> Seq.tryHead |> Option.defaultValue 0

    verticalReflection + horizontalReflection * 100

let verifyReflectionHasASingleSmudge encoded mirrorIx =
    let rec iter foundSmudge ix =
        if ix < 0 then
            foundSmudge
        else
            let mirroredIx = (mirrorIx - ix) + mirrorIx - 1
            
            if mirroredIx >= (encoded |> Array.length) then
                foundSmudge
            else
                let diff =
                    seq { 
                        for ci in 0..(encoded.[ix] |> Array.length) - 1 do
                            if encoded.[ix].[ci] = encoded.[mirroredIx].[ci] then
                                0
                            else
                                1
                    }
                    |> Seq.sum

                match diff with
                | 0 -> iter foundSmudge (ix - 1)
                | 1 when foundSmudge -> false
                | 1 -> iter true (ix - 1)
                | _ -> false

    iter false (mirrorIx - 1)

let findReflectionsPart2 encode (lavaIsland: LavaIsland) =
    let encoded = lavaIsland |> encode

    seq { for mix in 1..(encoded |> Array.length) - 1 do mix }
    |> Seq.filter (verifyReflectionHasASingleSmudge encoded)

[
    "Part 1", findReflectionsPart1
    "Part 2", findReflectionsPart2
]
|> List.iter (fun (part, findReflections) ->
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> splitByNewLines
    |> Seq.map (parseIntoMatrix (function
        | '#' -> R
        | '.' -> A
        | c -> failwithf "Unexpected character '%c'" c
    ))
    |> Seq.toArray
    |> Array.Parallel.sumBy (calculateReflections findReflections)
    |> printfn "%s: %A" part
)
