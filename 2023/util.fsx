open System
open System.IO
open System.Text.RegularExpressions

let readLines (filePath:string) = seq {
    use sr = new StreamReader (filePath)
    while not sr.EndOfStream do
        yield sr.ReadLine ()
}

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

let extractFirstLine lines =
    if lines |> Seq.isEmpty then failwith "Empty sequence"
    else lines |> Seq.head, lines |> Seq.skip 1
