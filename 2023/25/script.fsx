#load "../util.fsx"
open Util

open System.IO

let addEdge fromNode toNode graph =
    graph
    |> Map.change fromNode (
        Option.defaultValue List.empty
        >> (fun x -> toNode::x)
        >> Some
    )

let input =
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> Seq.fold (fun graph line ->
        let (fromNode, toInput) = line |> splitIntoTwo ':'
        let toNodes = toInput |> splitBy ' ' |> List.map _.Trim()

        toNodes
        |> List.fold (fun current toNode ->
            current
            |> addEdge fromNode toNode
            |> addEdge toNode fromNode
        ) graph
    ) Map.empty

let findPathToComponent fromNode toComponent withoutEdges graph =
    let rec iter sawNodes toProcess =
        match toProcess with
        | [] -> None
        | (p, es)::_ when toComponent |> Set.contains p ->
            es |> Some
        | (p, es)::rest ->
            let addToProcess =
                graph
                |> Map.find p
                |> List.map (fun tp ->
                    let e = Set [ p; tp ]
                    tp, e
                )
                |> List.filter (fst >> (fun tp -> sawNodes |> Set.contains tp |> not))
                |> List.filter (snd >> (fun e -> 
                    (withoutEdges |> Set.contains e || es |> Set.contains e)
                    |> not
                ))
                |> List.map (fun (tp, e) ->
                    (tp, es |> Set.add e)
                )
            
            let newSawNodes =
                addToProcess
                |> List.map fst
                |> Set.ofList
                |> Set.union sawNodes

            iter newSawNodes (addToProcess @ rest)
    iter (Set [ fromNode ]) [fromNode, Set.empty]

let rec isConnectedWithConnects fromNode toComponent connectionCount graph =
    let rec iter withoutEdges connectionCount =
        if connectionCount = 0 then
            true
        else
            match graph |> findPathToComponent fromNode toComponent withoutEdges with
            | None -> false
            | Some path ->
                iter (withoutEdges |> Set.union path) (connectionCount - 1)

    iter Set.empty connectionCount

let splitIntoTwoComponents connectionCount graph =
    graph
    |> Map.keys
    |> Seq.fold (fun (c1, c2) p ->
        if c1 |> Set.isEmpty || isConnectedWithConnects p c1 connectionCount graph then
            c1 |> Set.add p, c2
        else
            c1, c2 |> Set.add p
    ) (Set.empty, Set.empty)


input
|> splitIntoTwoComponents 4
|> fun (c1, c2) -> (c1 |> Set.count) * (c2 |> Set.count)
|> printfn "%A"