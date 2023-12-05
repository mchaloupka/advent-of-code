open System
open System.IO

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
