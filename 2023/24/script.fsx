#r "nuget: Microsoft.Z3, 4.8.14"

#load "../util.fsx"
open Util

open System.IO
open Microsoft.Z3;

type Vector = { X: int64; Y: int64; Z: int64 }

type Hailstone = { From: Vector; Movement: Vector }

let parseVector input =
    match input |> splitBy ',' |> List.map _.Trim() |> List.map int64 with
    | [x;y;z] -> { X = x; Y = y; Z = z }
    | _ -> failwithf "Unexpected input for vector: %A" input

let parseHailstone input =
    let (left, right) = input |> splitIntoTwo '@'
    {
        From = left |> parseVector
        Movement = right |> parseVector
    }

let areaFrom = 200000000000000.0
let areaTo = 400000000000000.0

let hailstoneIntersect h1 h2 =
    (*
        h1x + h1mx * a = h2x + h2mx * b
        h1y + h1my * a = h2y + h2my * b
    *)

    let h1x = h1.From.X |> double
    let h1mx = h1.Movement.X |> double
    let h2x = h2.From.X |> double
    let h2mx = h2.Movement.X |> double
    let h1y = h1.From.Y |> double
    let h1my = h1.Movement.Y |> double
    let h2y = h2.From.Y |> double
    let h2my = h2.Movement.Y |> double
    
    let a = (h2x - h1x + h2mx * h1y / h2my - h2mx * h2y / h2my) / (h1mx - h2mx / h2my * h1my)
    let b = (h1y + h1my * a - h2y) / h2my
    let x = h1x + h1mx * a
    let y = h1y + h1my * a

    x >= areaFrom && x <= areaTo && y >= areaFrom && y <= areaTo && a >= 0 && b >= 0

let input =
    Path.Combine(__SOURCE_DIRECTORY__, "input.txt")
    |> readLines
    |> Seq.map parseHailstone
    |> Seq.toList

let allPairs = input |> selfCartesian

allPairs
|> List.filter (fun (h1, h2) -> hailstoneIntersect h1 h2)
|> List.length
|> printfn "Part 1: %A"

// Using Z3 to solve the system of equations. https://github.com/Z3Prover/z3
let part2solution =
    let ctx = new Context()
    let solver = ctx.MkSolver()

    // Coordinates
    let x = ctx.MkIntConst("x")
    let y = ctx.MkIntConst("y")
    let z = ctx.MkIntConst("z")

    // Velocity
    let vx = ctx.MkIntConst("vx")
    let vy = ctx.MkIntConst("vy")
    let vz = ctx.MkIntConst("vz")

    // We need to use 3 hailstones so we have enough equations
    // for our unknown variables as each hailstone adds 3 equations
    // and one additional unknown variable.
    input
    |> List.take 3
    |> List.iteri (fun i h ->
        let t = ctx.MkIntConst(sprintf "t%d" i)
        let px = ctx.MkInt(h.From.X)
        let py = ctx.MkInt(h.From.Y)
        let pz = ctx.MkInt(h.From.Z)
        let mx = ctx.MkInt(h.Movement.X)
        let my = ctx.MkInt(h.Movement.Y)
        let mz = ctx.MkInt(h.Movement.Z)

        // t >= 0
        solver.Add(ctx.MkGe(t, ctx.MkInt(0)))

        // px + t * mx = x + t * vx
        solver.Add(
            ctx.MkEq(
                ctx.MkAdd(px, ctx.MkMul(t, mx)),
                ctx.MkAdd(x, ctx.MkMul(t, vx))
            )
        )

        // py + t * my = y + t * vy
        solver.Add(
            ctx.MkEq(
                ctx.MkAdd(py, ctx.MkMul(t, my)),
                ctx.MkAdd(y, ctx.MkMul(t, vy))
            )
        )

        // pz + t * mz = z + t * vz
        solver.Add(
            ctx.MkEq(
                ctx.MkAdd(pz, ctx.MkMul(t, mz)),
                ctx.MkAdd(z, ctx.MkMul(t, vz))
            )
        )
    )

    solver.Check() |> ignore

    let model = solver.Model

    [ x; y; z ]
    |> List.map (fun v -> model.Eval v)
    |> List.map _.ToString()
    |> List.map int64

part2solution
|> List.sum
|> printfn "Part 2: %A"