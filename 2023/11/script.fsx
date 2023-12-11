#load "../util.fsx"
open Util

open System.IO

type ImagePoint =
| E // Empty space
| G // Galaxy

type Space = ImagePoint array array

let getColumnOffsets offset (graph: Space) =
    let firstRow = graph.[0]

    let rec iter offsets prevOffset ci =
        if ci >= (firstRow |> Array.length) then
            offsets |> List.rev |> Array.ofList
        else
            if graph |> Array.forall (fun r -> r.[ci] = E) then
                iter (prevOffset::offsets) (prevOffset + offset) (ci + 1)
            else
                iter (prevOffset::offsets) prevOffset (ci + 1)
                 
    iter [] (bigint 0) 0

let getRowOffsets offset (graph: Space) =
    let rec iter offsets prevOffset ri =
        if ri >= (graph |> Array.length) then
            offsets |> List.rev |> Array.ofList
        else
            if graph.[ri] |> Array.forall (fun r -> r = E) then
                iter (prevOffset::offsets) (prevOffset + offset) (ri + 1)
            else
                iter (prevOffset::offsets) prevOffset (ri + 1)

    iter [] (bigint 0) 0

let calculateDistances offset (graph: Space) =
    let rowOffsets = graph |> getRowOffsets offset
    let columnOffsets = graph |> getColumnOffsets offset
    let galaxies = graph |> allIndices |> Seq.filter (fun (ri, ci) -> graph.[ri].[ci] = G) |> List.ofSeq

    galaxies
    |> allPairs
    |> List.map (fun ((x1, y1), (x2, y2)) ->
        let rDistance = (abs (x2 - x1) |> bigint) + abs (rowOffsets.[x2] - rowOffsets.[x1])
        let cDistance = (abs (y2 - y1) |> bigint) + abs (columnOffsets.[y2] - columnOffsets.[y1])

        rDistance + cDistance
    )
    |> List.sum

[
    1, 2 |> bigint
    2, 1000000 |> bigint
]
|> List.map (fun (part, multiplier) ->
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> parseIntoMatrix (function
        | '#' -> G
        | '.' -> E
        | c -> failwithf "Unexpected character '%c'" c
    )
    |> calculateDistances (multiplier - (bigint 1))
    |> printfn "Part %d: %A" part
)
