#!/usr/bin/julia

module SonarSweep

using RollingFunctions

f = open("input", "r")
nums = parse.(Int, readlines(f))

incs = count(diff(nums) .> 0)
println(incs)

sums = RollingFunctions.rolling(sum, nums, 3)
incs = count(diff(sums) .> 0)
println(incs)

end
