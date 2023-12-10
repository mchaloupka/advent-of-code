#load "../util.fsx"
open Util

open System.IO

let findNode node (graph: char array array) =
    let rec iterNodes ri ci =
        if graph.[ri].[ci] = node then
            ri, ci
        elif ci + 1 = (graph.[ri] |> Array.length) then
            if ri + 1 = (graph |> Array.length) then
                failwithf "Node %c not found in %A" node graph
            else
                iterNodes (ri + 1) 0
        else
            iterNodes ri (ci + 1)
    
    iterNodes 0 0

let rec getNextNodes (ri, ci) (graph: char array array) =
    if (ri < 0 || ri >= (graph |> Array.length) || ci < 0 || ci >= (graph.[ri] |> Array.length)) then
        List.empty
    else    
        match graph.[ri].[ci] with
        | '|' -> [ (ri-1, ci); (ri+1, ci) ]
        | '-' -> [ (ri, ci-1); (ri, ci+1) ]
        | 'L' -> [ (ri-1, ci); (ri, ci+1) ]
        | 'J' -> [ (ri-1, ci); (ri, ci-1) ]
        | '7' -> [ (ri, ci-1); (ri+1, ci) ]
        | 'F' -> [ (ri, ci+1); (ri+1, ci) ]
        | 'S' ->
            [ (ri, ci-1); (ri, ci+1); (ri-1, ci); (ri+1, ci) ] 
            |> List.filter (fun x -> 
                graph 
                |> getNextNodes x 
                |> List.exists (fun y -> y = (ri, ci))
            )
        | _ -> List.empty

let findFurthestAwayNode graph =
    let rec findFurthestAwayNode distances positions =
        match positions with
        | (n, d)::rest ->
            match distances |> Map.tryFind n with
            | Some bd when bd <= d ->
                findFurthestAwayNode distances rest
            | _ ->
                let newDistances =
                    distances
                    |> Map.add n d

                graph 
                |> getNextNodes n
                |> List.map (fun x -> x, d + 1)
                |> List.append positions
                |> findFurthestAwayNode newDistances
        | [] ->
            distances
            |> Map.values
            |> Seq.max
    
    let startNode = graph |> findNode 'S'
    findFurthestAwayNode Map.empty [startNode, 0]

Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
|> readLines
|> Seq.map (Seq.toArray)
|> Seq.toArray
|> findFurthestAwayNode
|> printfn "Part 1: %A"

type MovingAlong =
    | Top
    | Bottom
    | Left
    | Right
    | Any 

let markAllVisited neighbohours startNode graph =
    let rec iter marked nodes =
        match nodes with
        | n::rest ->
            if marked |> Set.contains n then
                iter marked rest
            else
                graph
                |> neighbohours n
                |> List.append rest
                |> iter (marked |> Set.add n)
        | [] -> marked
    
    iter Set.empty [startNode]

let calculateNotReachable inLoop reachableFromOutside (graph: char array array) =
    let rec calculate agg ri ci =
        let value =
            if inLoop |> Set.contains (ri, ci) then
                0 // On loop
            elif reachableFromOutside |> Set.contains (ri, ci) then
                0 // Reachable from outside
            else
                1 // Enclosed

        if ci + 1 = (graph.[ri] |> Array.length) then
            if ri + 1 = (graph |> Array.length) then
                agg + value
            else
                calculate (agg + value) (ri + 1) 0
        else
            calculate (agg + value) ri (ci + 1)

    calculate 0 0 0

let neighbohoursWithOutside inLoop ((ri, ci), movingAlong) (graph: char array array) =
    let below = ri + 1, ci
    let above = ri - 1, ci
    let onLeft = ri, ci - 1
    let onRight = ri, ci + 1
    
    let symbol =
        if inLoop |> Set.contains (ri, ci) then
            if graph.[ri].[ci] = 'S' then
                let aroundInLoop points =
                    points
                    |> List.forall (fun x -> 
                        if inLoop |> Set.contains x then
                            getNextNodes x graph
                            |> List.exists (fun y -> y = (ri, ci))
                        else
                            false
                    )
                
                if aroundInLoop [ below; above ] then
                    '|'
                elif aroundInLoop [ onLeft; onRight ] then
                    '-'
                elif aroundInLoop [ below; onLeft ] then
                    '7'
                elif aroundInLoop [ below; onRight ] then
                    'F'
                elif aroundInLoop [ above; onLeft ] then
                    'J'
                elif aroundInLoop [ above; onRight ] then
                    'L'
                else
                    failwith "Can't find start meaning"
            else
                graph.[ri].[ci]
        else
            '.'

    match movingAlong, symbol with
    | _, '.' ->
        [ 
            below, Top
            above, Bottom
            onLeft, Right
            onRight, Left
        ]
        |> List.filter (fun ((x, y), _) ->
            if ri < -1 || ri > (graph |> Array.length) then
                false
            elif ci < -1 || ci > (graph.[0] |> Array.length) then
                false
            else 
                true
        )
    | Left, '|' ->
        [ 
            below, Left
            above, Left
            onLeft, Right
        ]
    | Right, '|' ->
        [ 
            below, Right
            above, Right
            onRight, Left
        ]
    | Top, '-' ->
        [ 
            onLeft, Top
            onRight, Top
            above, Bottom
        ]
    | Bottom, '-' ->
        [ 
            onLeft, Bottom
            onRight, Bottom
            below, Top
        ]
    | Left, 'F'
    | Top, 'F' ->
        [
            above, Bottom
            onRight, Top
            onLeft, Right
            below, Left
        ]
    | Right, 'F'
    | Bottom, 'F' ->
        [
            onRight, Bottom
            below, Right
        ]
    | Left, 'L'
    | Bottom, 'L' ->
        [
            above, Left
            onRight, Bottom
            onLeft, Right
            below, Top
        ]
    | Right, 'L'
    | Top, 'L' ->
        [
            onRight, Top
            above, Right
        ]
    | Right, '7'
    | Top, '7' ->
        [
            above, Bottom
            onRight, Left
            onLeft, Top
            below, Right
        ]
    | Left, '7'
    | Bottom, '7' ->
        [
            onLeft, Bottom
            below, Left
        ]
    | Right, 'J'
    | Bottom, 'J' ->
        [
            above, Right
            onRight, Left
            onLeft, Bottom
            below, Top
        ]
    | Left, 'J'
    | Top, 'J' ->
        [
            onLeft, Top
            above, Left
        ]
    | _, _ -> failwithf "Can't do movement for %A - %A (%d:%d)" movingAlong symbol ri ci

let findEnclosedFields graph =
    let startNode = 
        graph 
        |> findNode 'S' 
    
    let inLoop = markAllVisited getNextNodes startNode graph
    let reachableFromOutside = 
        markAllVisited (neighbohoursWithOutside inLoop) ((-1, -1), Any) graph
        |> Set.map fst

    graph 
    |> calculateNotReachable inLoop reachableFromOutside

Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
|> readLines
|> Seq.map (Seq.toArray)
|> Seq.toArray
|> findEnclosedFields
|> printfn "Part 2: %A"