#load "../util.fsx"
open Util

open System
open System.IO

let processPart parseNumbers =
    let (times, distances) =
        Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
        |> readLines
        |> Seq.map (fun x -> x |> splitIntoTwo ':' |> snd |> splitBy ' ' |> parseNumbers)
        |> Seq.toArray
        |> fun x -> x.[0], x.[1]

    List.zip times distances
    |> List.map (fun (time, distance) ->
        // Using the following equation
        // x * (time - x) > distance
        // we are trying to find min and max x that is true for the above

        // We are operating with the whole numbers so we say that the distance needs to be at least 1 larger
        // -x^2 + x * time - (distance + 1) = 0 
        // D = time^2 - 4 * distance
        // x = (-time +- sqrroot(D)) / -2 = (time +- sqrroot(D)) / 2

        let D = (time * time - 4L * (distance + 1L)) |> double
        let minX = ceil (((time |> double) - sqrt D) / 2.0) |> int64
        let maxX = floor (((time |> double) + sqrt D) / 2.0) |> int64

        maxX - minX + 1L
    )
    |> List.fold (fun x y -> x * y) 1L
    

processPart (List.map int64) |> printfn "Part 1: %A"
processPart ((fun x -> String.Join(String.Empty, x |> List.toArray)) >> int64 >> List.singleton) |> printfn "Part 2: %A"
