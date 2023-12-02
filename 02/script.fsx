open System.IO

let readLines (filePath:string) = seq {
    use sr = new StreamReader (filePath)
    while not sr.EndOfStream do
        yield sr.ReadLine ()
}

let lines = 
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines

type Hand = { balls: Map<string, int> }
type Game = { id: int; hands: Hand list }

let parseLine (line: string) =
    let lineParts = 
        line.Split([|':'|], 2)
    let gameId = 
        lineParts.[0].Split([|' '|], 2).[1].Trim()
        |> int
    let hands =
        lineParts.[1].Split([|';'|])
        |> Array.toSeq
        |> Seq.map (fun handString -> 
            let balls = 
                handString.Split([|','|])
                |> Array.toSeq
                |> Seq.map (fun ballString ->
                    let ballStringParts = ballString.Trim().Split([|' '|])
                    let count = ballStringParts.[0].Trim() |> int
                    let colour = ballStringParts.[1].Trim()
                    colour, count
                )
                |> Map.ofSeq

            { balls = balls }
        )
        |> Seq.toList
    { id = gameId; hands = hands }

let games = lines |> Seq.map parseLine

let checkColorInHandAgainst colour limit balls =
    balls
    |> Map.tryFind colour
    |> Option.map (fun x -> x <= limit)
    |> Option.defaultValue true

let isPossibleHand hand =
    (hand.balls |> checkColorInHandAgainst "red" 12) &&
        (hand.balls |> checkColorInHandAgainst "blue" 14) &&
        (hand.balls |> checkColorInHandAgainst "green" 13)

let part1 = 
    games
    |> Seq.filter (fun game ->
        game.hands
        |> List.forall isPossibleHand
    )
    |> Seq.map _.id
    |> Seq.sum

printfn "Part 1: %d" part1

let getCount colour hand =
    hand.balls
    |> Map.tryFind colour
    |> Option.defaultValue 0

let mergeHands hand1 hand2 =

    let getMinimum colour =
        max (hand1 |> getCount colour) (hand2 |> getCount colour)

    let minBalls =
        ["red"; "green"; "blue"]
        |> List.map (fun colour -> colour, colour |> getMinimum)
        |> Map.ofList
    
    { balls = minBalls }

let part2 =
    games
    |> Seq.map (fun game ->
        let minBalls =
            game.hands
            |> List.reduce mergeHands

        minBalls
    )
    |> Seq.map (fun minHand ->
        ["red"; "green"; "blue"]
        |> List.map (fun x -> getCount x minHand)
        |> List.fold (fun power count -> power * count) 1
    )
    |> Seq.sum

printfn "Part 2: %d" part2
