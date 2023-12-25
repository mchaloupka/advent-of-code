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
    else lines |> Seq.head, lines |> Seq.tail

let rec splitByNewLines lines = seq {
    let rec gatherUntilNewLine agg groupLines = 
        if groupLines |> Seq.isEmpty then
            Seq.empty, agg |> List.rev
        else
            let firstLine = groupLines |> Seq.head
            let remaining = groupLines |> Seq.tail

            if (firstLine = "") then
                remaining, agg |> List.rev
            else
                gatherUntilNewLine (firstLine::agg) remaining

    if lines |> Seq.isEmpty |> not then
        let (remainingLines, group) = gatherUntilNewLine [] lines
        yield group
        yield! splitByNewLines remainingLines
    else
        ()
}

let parseIntoMatrix charParser lines =
    lines
    |> Seq.map (Seq.map charParser >> Seq.toArray)
    |> Seq.toArray

// Parsing operations
let splitIntoTwo character (text: string) =
    let split = text.Split([|character|])
    split.[0], split.[1]

let splitBy (character: char) (text: string) =
    let split = text.Split([|character|], StringSplitOptions.RemoveEmptyEntries)
    split |> List.ofArray

let splitByRegexGroups (regex: string) (text:string) =
    let rx: Regex = Regex(regex, RegexOptions.None)
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

// Numerical functions
let euclideanMod a b = ((a % b) + b) % b

// Combination number n over k
let comb n k =
    let rec perfComb i agg1 agg2 =
        if i > n - k then agg1 / agg2
        else perfComb (i + 1) (agg1 * (bigint (n - i + 1))) (agg2 * (bigint i))

    perfComb 1 (bigint 1) (bigint 1) |> int64

// All pairs from list
let allPairs input =
    input
    |> List.mapi (fun i x -> x, i)
    |> List.collect (fun (x, i1) ->
        input
        |> List.mapi (fun i2 y ->
            if (i2 > i1) then
                (x, y) |> Some
            else
                None
        )
        |> List.choose id
    )

// Array function
let allIndices<'T> (array2d: 'T array array) =
    let rec iter ri ci = seq {
        yield (ri, ci)
        if ci + 1 = (array2d.[ri] |> Array.length) then
            if ri + 1 < (array2d |> Array.length) then
                yield! iter (ri + 1) 0
        else
            yield! iter ri (ci + 1)
    }

    iter 0 0

let printMatrix<'T> (transform: (int * int) -> char) (array2d: 'T array array) =
    let rec iter ri ci =
        (ri, ci) |> transform |> printf "%c"
        
        if ci + 1 = (array2d.[ri] |> Array.length) then
            printfn ""

            if ri + 1 < (array2d |> Array.length) then
                iter (ri + 1) 0
        else
            iter ri (ci + 1)

    iter 0 0

let cartesian xs ys = 
    xs |> List.collect (fun x -> ys |> List.map (fun y -> x, y))

let selfCartesian xs =
    let withIndexes = xs |> List.mapi (fun i x -> i, x)
    withIndexes
    |> List.collect (fun (i, x) ->
        withIndexes
        |> List.filter (fun (ii, _) -> ii > i)
        |> List.map (fun (_, y) -> x, y)
    )
    