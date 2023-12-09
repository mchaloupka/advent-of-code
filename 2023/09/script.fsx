#load "../util.fsx"
open Util

open System.IO

// Numbers are reversed start of the prediction lines; from bottom to up
let nextPredictedNumber numberIndex =
    let rec nextNumber index agg numbers =
        match numbers with
        | [] -> agg
        | x::rest ->
            // How many times the number is applied is based on Pascal's triangle
            // therefore, it is (numberIndex \over index)
            let appliedTimes = comb numberIndex index
            let numberInPlace = x * appliedTimes + agg
            
            nextNumber (index + 1) numberInPlace rest

    nextNumber 0 0

let getNextNumber =
    let rec predictNext predictionNumbers numberIndex numbers =
        match numbers with
        | x::rest ->
            let nextPredictedNumber = nextPredictedNumber numberIndex (0L::predictionNumbers)
            let diffFromPrediction = x - nextPredictedNumber
            predictNext (diffFromPrediction::predictionNumbers) (numberIndex + 1) rest
        | [] -> nextPredictedNumber numberIndex (0::predictionNumbers)
    predictNext [] 0

[
    1, id
    // The part 2 is the same as part 1, we just 
    // take the numbers in reverse order to predict the beginning instead of the end
    2, List.rev 
]
|> List.iter (fun (part, inputTransform) ->
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> Seq.map (splitBy ' ' >> List.map int64 >> inputTransform)
    |> Seq.map getNextNumber
    |> Seq.sum
    |> printfn "Part %d: %A" part 
)
