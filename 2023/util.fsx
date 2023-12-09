open System
open System.IO
open System.Text.RegularExpressions

// File input operation
let readLines (filePath:string) = seq {
    use sr = new StreamReader (filePath)
    while not sr.EndOfStream do
        yield sr.ReadLine ()
}

let extractFirstLine lines =
    if lines |> Seq.isEmpty then failwith "Empty sequence"
    else lines |> Seq.head, lines |> Seq.skip 1

// Parsing operations
let splitIntoTwo character (text: string) =
    let split = text.Split([|character|])
    split.[0], split.[1]

let splitBy (character: char) (text: string) =
    let split = text.Split([|character|], StringSplitOptions.RemoveEmptyEntries)
    split |> List.ofArray

let splitByRegexGroups (regex: string) (text:string) =
    let rx = Regex(regex, RegexOptions.None)
    let m = rx.Match text
    
    if m.Success then m.Groups |> Seq.skip 1 |> Seq.map (fun x -> x.Value) |> Seq.toList
    else failwithf "Regex '%s' does not match '%s'" regex text

// Function to calculate the Greatest Common Divisor
let rec gcd a b =
    if b = 0L then a
    else gcd b (a % b)

// Function to calculate the Least Common Multiple
let lcm a b =
    (a * b) / gcd a b

// Combination number n over k
let comb n k =
    let rec perfComb i agg1 agg2 =
        if i > n - k then agg1 / agg2
        else perfComb (i + 1) (agg1 * (bigint (n - i + 1))) (agg2 * (bigint i))

    perfComb 1 (bigint 1) (bigint 1) |> int64
