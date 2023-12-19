#load "../util.fsx"
open Util

open System
open System.IO

type ItemProperty = | X | M | A | S

let parseItemProperty input =
    match input with
    | "x" -> X
    | "m" -> M
    | "a" -> A
    | "s" -> S
    | _ -> failwithf "Unknown property %s" input

type Item = Map<ItemProperty, int>

let parseItem line =
    match line |> splitByRegexGroups @"{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}" with
    | [x;m;a;s] -> Map [ (X, x |> int); (M, m |> int); (A, a |> int); (S, s |> int) ]
    | _ -> failwithf "Cannot parse item from %s" line

let itemValue (item: Item) =
    item
    |> Map.toSeq
    |> Seq.map snd
    |> Seq.sum
    |> bigint

type ConditionComparison = | L | G

type ConditionRule = {
    Property: ItemProperty
    Value: int
    Comparison: ConditionComparison
}

type ConditionedTransition = {
    Condition: ConditionRule
    Target: string
}

type MachineState = {
    Name: string
    Transitions: ConditionedTransition list
    OtherwiseTarget: string
}

type Machine = Map<string, MachineState>

let parseMachine input =
    input
    |> Seq.fold (fun curMachine line ->
        let (name, rest) = line |> splitIntoTwo '{'
        let rules = rest.[0..rest.Length - 2] |> splitBy ',' |> List.rev
        let otherwiseTarget = rules |> List.head
        let transitions = 
            rules
            |> List.tail
            |> List.rev
            |> List.map (fun entry ->
                let (conditionString, target) = entry |> splitIntoTwo ':'
                let condition =
                    if conditionString.Contains('<') then
                        conditionString |> splitIntoTwo '<', L
                    elif conditionString.Contains('>') then       
                        conditionString |> splitIntoTwo '>', G
                    else
                        failwithf "Cannot parse transition %s" entry
                    |> fun ((property, valRaw), comp) ->
                        {
                            Property = property |> parseItemProperty
                            Value = valRaw |> int
                            Comparison = comp
                        }
                
                {
                    Condition = condition
                    Target = target
                }
            )

        curMachine |> Map.add name {
            Name = name
            Transitions = transitions
            OtherwiseTarget = otherwiseTarget
        }
    ) Map.empty

let transitionAcceptsItem (item: Item) (transition: ConditionedTransition) =
    let value = item |> Map.find transition.Condition.Property

    match transition.Condition.Comparison with
    | L -> value < transition.Condition.Value
    | G -> value > transition.Condition.Value

let acceptedByMachine (machine: Machine) (item: Item) =
    let rec iter curState =
        if curState = "A" then
            true
        elif curState = "R" then
            false
        else
            let state = machine |> Map.find curState
            let nextState =
                state.Transitions 
                |> List.tryFind (transitionAcceptsItem item)
                |> Option.map _.Target
                |> Option.defaultValue state.OtherwiseTarget
            iter nextState

    iter "in"

let (machine, items) =
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> splitByNewLines
    |> extractFirstLine
    |> fun (machineInput, itemInput) ->
        let machine = machineInput |> parseMachine
        let items =
            itemInput
            |> Seq.head
            |> List.map parseItem

        machine, items

items
|> List.filter (acceptedByMachine machine)
|> List.map itemValue
|> List.sum
|> printfn "Part 1: %A"

type ItemRange = Map<ItemProperty, int * int>

let isValid (itemRange: ItemRange) =
    itemRange
    |> Map.forall (fun _ (rangeFrom, rangeTo) -> rangeFrom < rangeTo)

let rangeValue (itemRange: ItemRange) =
    itemRange
    |> Map.toSeq
    |> Seq.map snd
    |> Seq.map (fun (rangeFrom, rangeTo) -> rangeTo - rangeFrom + 1 |> bigint)
    |> Seq.fold (fun agg x -> agg * x) (bigint 1)

let getTransitionedRanges (state: MachineState) input =
    let rec iter agg nextItems curItems transitions =
        match curItems, transitions with
        | item::rest, _ when item |> isValid |> not -> iter agg nextItems rest transitions
        | [], [] -> nextItems, agg
        | item::rest, [] -> iter agg (item::nextItems) rest transitions
        | [], _::rest -> iter agg List.empty nextItems rest
        | item::rest, transition::_ ->
            let (rangeFrom, rangeTo) = item |> Map.find transition.Condition.Property

            let newItem newRangeFrom newRangeTo =
                item |> Map.add transition.Condition.Property (newRangeFrom, newRangeTo)

            match transition.Condition.Comparison with
            | L -> 
                let newRangeTo = min rangeTo (transition.Condition.Value - 1)
                iter ((transition.Target, newItem rangeFrom newRangeTo)::agg) ((newItem (newRangeTo + 1) rangeTo)::nextItems) rest transitions
            | G ->
                let newRangeFrom = max rangeFrom (transition.Condition.Value + 1)
                iter ((transition.Target, newItem newRangeFrom rangeTo)::agg) ((newItem rangeFrom (newRangeFrom - 1))::nextItems) rest transitions

    let (remainingRanges, foundTransitions) = iter List.empty List.empty [ input ] state.Transitions
    
    remainingRanges
    |> List.map (fun x -> state.OtherwiseTarget, x)
    |> List.append foundTransitions

let getAcceptedRanges (machine: Machine) =
    let rec iter (agg: ItemRange list) toProcess =
        match toProcess with
        | (_, item)::rest when item |> isValid |> not -> iter agg rest
        | ("A", item)::rest -> iter (item::agg) rest
        | ("R", _)::rest -> iter agg rest
        | (curState, curItem)::rest ->
            let state = machine |> Map.find curState
            
            getTransitionedRanges state curItem
            |> List.append rest
            |> iter agg
        | [] -> agg

    ("in", (Map [(X, (1, 4000)); (M, (1, 4000)); (A, (1, 4000)); (S, (1, 4000))]))
    |> List.singleton
    |> iter List.empty

machine
|> getAcceptedRanges
|> List.map rangeValue
|> List.sum
|> printfn "%A"
