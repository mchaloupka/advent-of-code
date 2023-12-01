open System
open System.IO

let readLines (filePath:string) = seq {
    use sr = new StreamReader (filePath)
    while not sr.EndOfStream do
        yield sr.ReadLine ()
}

let lines = 
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines

let rec getNumbers accInput accOutput input =
    match input with
    | c :: rest ->
        if (c |> Char.IsDigit) then
            getNumbers String.Empty (c :: accOutput) rest
        else
            match accInput, c with
            | "o", 'n' -> getNumbers "on" accOutput rest
            | "on", 'e' -> getNumbers "e" ('1' :: accOutput) rest
            | "t", 'w' -> getNumbers "tw" accOutput rest
            | "tw", 'o' -> getNumbers "o" ('2' :: accOutput) rest
            | "t", 'h' -> getNumbers "th" accOutput rest
            | "th", 'r' -> getNumbers "thr" accOutput rest
            | "thr", 'e' -> getNumbers "thre" accOutput rest
            | "thre", 'e' -> getNumbers "e" ('3' :: accOutput) rest
            | "f", 'o' -> getNumbers "fo" accOutput rest
            | "fo", 'u' -> getNumbers "fou" accOutput rest
            | "fou", 'r' -> getNumbers String.Empty ('4' :: accOutput) rest
            | "f", 'i' -> getNumbers "fi" accOutput rest
            | "fi", 'v' -> getNumbers "fiv" accOutput rest
            | "fiv", 'e' -> getNumbers "e" ('5' :: accOutput) rest
            | "s", 'i' -> getNumbers "si" accOutput rest
            | "si", 'x' -> getNumbers String.Empty ('6' :: accOutput) rest
            | "s", 'e' -> getNumbers "se" accOutput rest
            | "se", 'v' -> getNumbers "sev" accOutput rest
            | "sev", 'e' -> getNumbers "seve" accOutput rest
            | "seve", 'n' -> getNumbers "n" ('7' :: accOutput) rest
            | "e", 'i' -> getNumbers "ei" accOutput rest
            | "ei", 'g' -> getNumbers "eig" accOutput rest
            | "eig", 'h' -> getNumbers "eigh" accOutput rest
            | "eigh", 't' -> getNumbers "t" ('8' :: accOutput) rest
            | "n", 'i' -> getNumbers "ni" accOutput rest
            | "ni", 'n' -> getNumbers "nin" accOutput rest
            | "nin", 'e' -> getNumbers "e" ('9' :: accOutput) rest
            | "", _ -> getNumbers (c |> string) accOutput rest
            | nonEmpty, _ -> getNumbers (nonEmpty.[1..]) accOutput input       
    | [] -> accOutput |> List.rev

let withCalibrationValues getDigits =
    lines
    |> Seq.map (fun line ->
        let digits =
            line |> Seq.toList |> getDigits |> Seq.toArray
        let firstDigit = digits |> Seq.head
        let lastDigit = digits |> Seq.rev |> Seq.head
        line, sprintf "%c%c" firstDigit lastDigit |> int
    )

let getTotal getDigits =
    withCalibrationValues getDigits
    |> Seq.map snd
    |> Seq.sum

getTotal (fun chars -> chars |> List.filter Char.IsDigit)
|> printfn "Part1: %d"

getTotal (getNumbers String.Empty [])
|> printfn "Part2: %d"
