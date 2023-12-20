#load "../util.fsx"
open Util

open System
open System.IO

type ModuleType =
| FF
| CON
| BC

type Module = { Name: string; ModuleType: ModuleType; Outputs: string list; }

type Machine = { Connections: Map<string, Module>; FlipFlopsOn: Set<string>; LastInputs: Map<string, Map<string, bool>> }

let parseMachine =
    Seq.fold (fun machine line ->
        let (left, right) = line |> splitIntoTwo '>'
        let (name, moduleType) =
            match left.[0] with
            | 'b' -> "broadcaster", BC
            | '%' -> left.[1..left.Length - 3], FF
            | '&' -> left.[1..left.Length - 3], CON
            | _ -> failwithf "Invalid module input %s" left

        let outputs = right |> splitBy ',' |> List.map _.Trim()
        
        let updatedConnections = 
            machine.Connections 
            |> Map.add  name { Name = name; ModuleType = moduleType; Outputs = outputs }

        let updatedLastInputs =
            outputs
            |> List.fold (fun lastInputs output ->
                lastInputs
                |> Map.change output ((Option.defaultValue Map.empty) >> (Map.add name false) >> Some)
            ) machine.LastInputs

        { machine with
            Connections = updatedConnections
            LastInputs = updatedLastInputs
        }       
    ) { Connections = Map.empty; FlipFlopsOn = Set.empty; LastInputs = Map [ ("broadcaster", Map [ ("button", false) ])] }

type MachineOutput = { High: bigint; Low: bigint }

let addSignal isHigh output =
    if isHigh then
        { output with High = output.High + bigint 1 }
    else
        { output with Low = output.Low + bigint 1 }

let getOutputValue output = output.High * output.Low

let processButtonPress signalCounter =
    let rec iter output toProcess machine =
        match toProcess with
        | [] -> output, machine
        | (fromName, toName, isHigh)::rest ->
            // printfn "Processing %s->%s (%b)" fromName toName isHigh

            let newOutput = output |> signalCounter toName isHigh
            let newLastInputs = 
                machine.LastInputs 
                |> Map.change toName (fun lastInputs ->
                    lastInputs
                    |> Option.get
                    |> Map.add fromName isHigh
                    |> Some
                )

            let outputSignals, flipFlopsOn =
                match machine.Connections |> Map.tryFind toName with
                | Some curModule ->
                    match curModule.ModuleType with
                    | BC -> curModule.Outputs |> List.map (fun next -> next, isHigh), machine.FlipFlopsOn
                    | FF ->
                        if isHigh then
                            List.empty, machine.FlipFlopsOn
                        elif machine.FlipFlopsOn |> Set.contains toName then
                            curModule.Outputs |> List.map (fun next -> next, false), machine.FlipFlopsOn |> Set.remove toName
                        else
                            curModule.Outputs |> List.map (fun next -> next, true), machine.FlipFlopsOn |> Set.add toName
                    | CON ->
                        if newLastInputs |> Map.find toName |> Map.forall (fun _ x -> x) then
                            curModule.Outputs |> List.map (fun next -> next, false), machine.FlipFlopsOn
                        else
                            curModule.Outputs |> List.map (fun next -> next, true), machine.FlipFlopsOn
                | None ->
                    // Test module, do nothing
                    List.empty, machine.FlipFlopsOn

            let newToProcess =
                outputSignals
                |> List.map (fun (next, isHigh) -> (toName, next, isHigh))
                |> List.append rest

            iter newOutput newToProcess { machine with FlipFlopsOn = flipFlopsOn; LastInputs = newLastInputs }

    iter { High = bigint 0; Low = bigint 0 } ["button", "broadcaster", false]

let processButtonPresses =
    let rec iter performedCycles curOutput desiredCount machine =
        if performedCycles = desiredCount then
            curOutput
        else
            let (thisOutput, nextMachine) = machine |> processButtonPress (fun _ isHigh -> addSignal isHigh)
            let cycleCount = performedCycles + 1
            let newOutput = { High = curOutput.High + thisOutput.High; Low = curOutput.Low + thisOutput.Low }

            iter cycleCount newOutput desiredCount nextMachine
    
    iter 0 { High = bigint 0; Low = bigint 0 }

let machine =
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> parseMachine

machine
|> processButtonPresses 1000
|> getOutputValue
|> printfn "Part 1: %A"

let getEmitLoop stateName machine =
    let inputTree =
        let rec iter agg names =
            match names with
            | [] -> agg
            | x::rest when agg |> Set.contains x -> iter agg rest
            | x::rest when x = "button" -> iter agg rest
            | x::rest ->
                let inputs = 
                    machine.LastInputs 
                    |> Map.tryFind x 
                    |> Option.defaultWith (fun () -> failwithf "Can't find %s in last inputs %A" x machine.LastInputs)
                    |> Map.toList 
                    |> List.map fst
                iter (agg |> Set.add x) (inputs @ rest)

        iter Set.empty [stateName]

    let getInputTreeState fromMachine =
        let flipFlopsOn = 
            fromMachine.FlipFlopsOn 
            |> Set.intersect inputTree
        
        let inputStates = 
            inputTree
            |> Set.filter (fun state -> fromMachine.Connections |> Map.find state |> function | { ModuleType = CON } -> true | _ -> false)
            |> Set.fold (fun agg state -> 
                agg 
                |> Map.add state (fromMachine.LastInputs |> Map.find state)
            ) Map.empty

        flipFlopsOn, inputStates

    let rec iter inputStatesFound cycleOutputs machine =
        let (thisOutput, nextMachine) = 
            machine 
            |> processButtonPress (fun name isHigh ->
                if name = stateName then addSignal isHigh
                else id
            )

        let curState = nextMachine |> getInputTreeState
        let nextOutputs = thisOutput::cycleOutputs
        let curIx = nextOutputs |> List.length

        match inputStatesFound |> Map.tryFind curState with
        | Some ix ->
            if ix = 1 then
                cycleOutputs |> List.rev
            else
                failwithf "Not supporting cycles not starting from beginning"
        | None ->
            iter (inputStatesFound |> Map.add curState curIx) nextOutputs nextMachine

    iter Map.empty List.empty machine 

let neededButtonPressToEmitLow stateName machine =
    let inputs = machine.LastInputs |> Map.find stateName
    
    // This algorithm is tailored for the input, it is not a generic solution.
    // Input of "rx" is a single conjunction. To emit a single low from it, we need to get to the state
    // that in the processing the conjunction gets to the state of having all inputs high and that is when the processing have to end.
    // This does not happen in the first loop when we emit multiple times. So, we look on the inputs of the CON and 
    // identify loops of their emitting. In the loop, they keep emitting highs except for the last process of the loop where they emit low.
    // That allows the emit happen only when the loops align.
    if inputs |> Map.count > 1 then
        failwithf "Can't process module with multiple inputs"
    else
        let input = inputs |> Map.toSeq |> Seq.head |> fst

        match machine.Connections |> Map.find input with
        | { ModuleType = CON } as m ->
            let conInputs = machine.LastInputs |> Map.find m.Name |> Map.toList |> List.map fst
            
            let loopLenghts =
                conInputs
                |> List.map (fun input -> 
                    let loop = getEmitLoop input machine

                    if loop |> List.rev |> List.tail |> List.forall (fun x -> x.Low = bigint 0 && x.High + x.Low > bigint 0) then
                        loop |> List.length
                    else
                        failwithf "We are potentially emitting a single low also outside of the loop end: %A" loop
                )

            loopLenghts
            |> List.map int64
            |> List.reduce lcm
        | _ ->
            failwithf "Does not handle other input"
   

machine
|> neededButtonPressToEmitLow "rx"
|> printfn "Part 2: %A"
