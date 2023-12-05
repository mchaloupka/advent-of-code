#load "../util.fsx"
open Util
open System
open System.IO

type RangeEntry = { DestinationFrom: int64; SourceFrom: int64; Range: int64 }

type Map = { 
    Seeds: List<int64>
    SeedToSoil: List<RangeEntry>
    SoilToFeritilizer: List<RangeEntry>
    FertilizerToWater: List<RangeEntry>
    WaterToLight: List<RangeEntry>
    LightToTemperature: List<RangeEntry>
    TemperatureToHumidity: List<RangeEntry>
    HumidityToLocation: List<RangeEntry>
}

let emptyMap = { 
    Seeds = List.empty
    SeedToSoil = List.empty
    SoilToFeritilizer = List.empty
    FertilizerToWater = List.empty
    WaterToLight = List.empty
    LightToTemperature = List.empty
    TemperatureToHumidity = List.empty
    HumidityToLocation = List.empty
}

let map =
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> Seq.fold (fun (map, updateFunc) line ->
        if line.StartsWith("seeds:") then
            let (_, seeds) = line |> splitIntoTwo ':'
            { map with Seeds = map.Seeds |> List.append (seeds |> splitBy ' ' |> List.map int64) }, updateFunc
        elif String.IsNullOrWhiteSpace(line) then map, updateFunc
        else
            match line with
            | "seed-to-soil map:" -> (map, fun x m -> { m with SeedToSoil = x :: m.SeedToSoil })
            | "soil-to-fertilizer map:" -> (map, fun x m -> { m with SoilToFeritilizer = x :: m.SoilToFeritilizer })   
            | "fertilizer-to-water map:" -> (map, fun x m -> { m with FertilizerToWater = x :: m.FertilizerToWater })
            | "water-to-light map:" -> (map, fun x m -> { m with WaterToLight = x :: m.WaterToLight })
            | "light-to-temperature map:" -> (map, fun x m -> { m with LightToTemperature = x :: m.LightToTemperature })
            | "temperature-to-humidity map:" -> (map, fun x m -> { m with TemperatureToHumidity = x :: m.TemperatureToHumidity })
            | "humidity-to-location map:" -> (map, fun x m -> { m with HumidityToLocation = x :: m.HumidityToLocation })
            | _ ->
                let rangeParts = line |> splitBy ' ' |> List.map int64
                map |> updateFunc { DestinationFrom = rangeParts.[0]; SourceFrom = rangeParts.[1]; Range = rangeParts.[2] }, updateFunc
    ) (emptyMap, fun _ _ -> failwith "should not happen")
    |> fst

let getDestination source ranges =
    ranges
    |> List.tryFind (fun r -> r.SourceFrom <= source && (r.SourceFrom + r.Range) > source)
    |> Option.map (fun r -> source - r.SourceFrom + r.DestinationFrom)
    |> Option.defaultValue source

let rec findLocations input transformed transformations =
    match input, transformed, transformations with
    | _, _, [] -> input
    | [], _, _::rest -> findLocations transformed [] rest
    | i::rest, _, t::_ ->
        findLocations rest ((getDestination i t)::transformed) transformations

let allTransformations = [
    map.SeedToSoil
    map.SoilToFeritilizer
    map.FertilizerToWater
    map.WaterToLight
    map.LightToTemperature
    map.TemperatureToHumidity
    map.HumidityToLocation
]

findLocations map.Seeds [] allTransformations
|> List.min
|> printfn "Part 1: %A"

type SeedRange = { From: int64; Length: int64 }

let rec seedsAsRanges acc seeds =
    match seeds with
    | f::t::rest -> seedsAsRanges ({ From = f; Length = t }::acc) rest
    | [] -> acc
    | _ -> failwith "Seeds were not in pairs"

let isValidRange range =
    range.Length > 0

let rec getDestinationForRanges output sources checkedsources ranges =
    match sources, checkedsources, ranges with
    | _, _, [] -> output |> List.append sources |> List.append checkedsources
    | [], _, _::rest -> getDestinationForRanges output checkedsources [] rest
    | x::rest, _, r::_ ->
        let remainOnLeft = { From = x.From; Length = min x.Length (r.SourceFrom - x.From) }
        let remainOnRight = { From = max x.From (r.SourceFrom + r.Range); Length = min x.Length (x.From + x.Length - r.SourceFrom - r.Range) }
        let overlap = 
            let overlapFrom = max x.From r.SourceFrom
            let overlapToExclusive = min (x.From + x.Length) (r.SourceFrom + r.Range)
            { From = overlapFrom; Length = overlapToExclusive - overlapFrom }

        let remains = [ remainOnLeft; remainOnRight ] |> List.filter isValidRange

        let updatedOutput = 
            if overlap |> isValidRange then
                { overlap with From = overlap.From - r.SourceFrom + r.DestinationFrom }::output
            else
                output

        getDestinationForRanges updatedOutput rest (List.append remains checkedsources) ranges

let rec findLocationsForRanges input transformed transformations =
    match input, transformations with
    | _, [] -> input
    | [], _::rest -> 
        findLocationsForRanges transformed [] rest
    | input, t::_ ->
        findLocationsForRanges [] (List.append (getDestinationForRanges [] input [] t) transformed) transformations

findLocationsForRanges (map.Seeds |> seedsAsRanges []) [] allTransformations
|> List.filter (fun x -> x.Length > 0)
|> List.minBy _.From
|> fun x -> printfn "Part 2: %A" x.From
