open System
open System.IO

// Adding empty line before and after to ensure we have a row to look before and after
let readLines (filePath:string) = seq {
    yield String.Empty

    use sr = new StreamReader (filePath)
    while not sr.EndOfStream do
        yield sr.ReadLine ()

    yield String.Empty
}

let lines = 
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines

type Character =
    | Symbol of char
    | Nothing
    | Digit of int

let parseLine line =
    line
    |> Seq.map (fun c ->
        if (c |> Char.IsDigit) then
            c |> Char.GetNumericValue |> int |> Digit
        elif c = '.' then
            Nothing
        else
            Symbol c
    )
    |> Seq.toArray

let parsedLines = lines |> Seq.map parseLine

let zippedLines = 
    parsedLines 
    |> Seq.windowed 3
    |> Seq.map (fun x -> x.[0], x.[1], x.[2])

let isSymbol index line =
    if index >= (line |> Array.length) then false
    elif index < 0 then false
    else 
        match line.[index] with
        | Symbol _ -> true
        | _ -> false

let isAroundSymbol index line =
    [index - 1; index; index + 1] |> List.exists (fun x -> line |> isSymbol x)

let isAroundSymbolOnAnyLine index lines =
    lines |> List.exists (isAroundSymbol index)

let getAllNumbers (prevLine, thisLine, nextLine) =
    let rec getNumbers curNumber isNextToSymbol returnNumbers index =
        if index = (thisLine |> Array.length) then
            if isNextToSymbol then
                (curNumber :: returnNumbers) |> List.rev
            else
                returnNumbers |> List.rev
        else
            match thisLine.[index] with
            | Symbol _
            | Nothing ->
                if isNextToSymbol then
                    getNumbers 0 false (curNumber :: returnNumbers) (index + 1)
                else
                    getNumbers 0 false returnNumbers (index + 1)
            | Digit i ->
                if isNextToSymbol then
                    getNumbers (curNumber * 10 + i) isNextToSymbol returnNumbers (index + 1)
                else
                    getNumbers (curNumber * 10 + i) (isAroundSymbolOnAnyLine index [prevLine; thisLine; nextLine]) returnNumbers (index + 1)
    getNumbers 0 false [] 0

let sumOfAllNumbers =
    zippedLines
    |> Seq.map getAllNumbers
    |> Seq.map (List.sum)
    |> Seq.sum

printfn "Part 1: %d" sumOfAllNumbers

let getAllNumbersWithIndexes (prevLine, thisLine, nextLine) =
    let rec getNumbers curNumberStart curNumber isNextToSymbol returnNumbers index =
        if index = (thisLine |> Array.length) then
            if isNextToSymbol then
                ((curNumber, curNumberStart, index - 1) :: returnNumbers) |> List.rev
            else
                returnNumbers |> List.rev
        else
            match thisLine.[index] with
            | Symbol _
            | Nothing ->
                if isNextToSymbol then
                    getNumbers (index + 1) 0 false ((curNumber, curNumberStart, index - 1) :: returnNumbers) (index + 1)
                else
                    getNumbers (index + 1) 0 false returnNumbers (index + 1)
            | Digit i ->
                if isNextToSymbol then
                    getNumbers curNumberStart (curNumber * 10 + i) isNextToSymbol returnNumbers (index + 1)
                else
                    getNumbers curNumberStart (curNumber * 10 + i) (isAroundSymbolOnAnyLine index [prevLine; thisLine; nextLine]) returnNumbers (index + 1)
    getNumbers 0 0 false [] 0

let getAllGears ((_, prevNumbers), (thisLine, thisNumbers), (_, nextNumbers)) =
    let rec getGears index gears =
        if index = (thisLine |> Array.length) then
            gears |> List.rev
        else
            match thisLine.[index] with
            | Symbol '*' ->
                let numbersAround = 
                    [prevNumbers; thisNumbers; nextNumbers]
                    |> List.concat
                    |> List.filter (fun (_, indexFrom, indexTo) ->
                        (index >= indexFrom - 1 && index <= indexTo + 1)
                    )
                    |> List.map (fun (number, _, _) -> number)

                if numbersAround |> List.length >= 2 then
                    let value = numbersAround |> List.fold (fun agg x -> agg * x) 1
                    getGears (index + 1) (value :: gears)
                else
                    getGears (index + 1) gears
            | _ -> getGears (index + 1) gears
    getGears 0 []

let linesWithParsedNumbers =
    zippedLines
    |> Seq.map (fun (prev, this, next) -> this, (prev, this, next) |> getAllNumbersWithIndexes)
    |> Seq.windowed 3
    |> Seq.map (fun x -> x.[0], x.[1], x.[2])
    |> Seq.map getAllGears
    |> Seq.map List.sum
    |> Seq.sum

printfn "Part 2: %d" linesWithParsedNumbers
