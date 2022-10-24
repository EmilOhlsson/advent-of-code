#!/usr/bin/julia


module Dive

using Match

function part1(captures)
    pos = (0, 0)
    for m in captures
        d = parse(Int, m["dist"])
        pos = pos .+ @match m["dir"] begin
            "forward" => (d, 0)
            "down" => (0, d)
            "up" => (0, -d)
        end
    end
    return pos[1] * pos[2]
end

function part2(captures)
    pos = (0, 0, 0) # Aim, horizontal distance, depth
    for m in captures
        d = parse(Int, m["dist"])
        pos = pos .+ @match m["dir"] begin
            "forward" => (0, d, d * pos[1])
            "down" => (d, 0, 0)
            "up" => (-d, 0, 0)
        end
    end
    return pos[2] * pos[3]
end

f = open("input", "r")
captures = match.(r"^(?<dir>forward|down|up) (?<dist>\d+)$", readlines(f))

println(part1(captures))
println(part2(captures))

end
