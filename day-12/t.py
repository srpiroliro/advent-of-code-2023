def count(cfg, nums):
    if cfg == "":
        return 1 if nums == () else 0

    if nums == ():
        return 0 if "#" in cfg else 1

    result = 0
    
    if cfg[0] in ".?":
        result += count(cfg[1:], nums)
        
    if cfg[0] in "#?":
        if nums[0] <= len(cfg) and "." not in cfg[:nums[0]] and (nums[0] == len(cfg) or cfg[nums[0]] != "#"):
            print(cfg, nums)
            result += count(cfg[nums[0]:], nums[1:])

    return result

total = 0

# open("input.txt").read()

lines="""???.### 1,1,3""".strip().splitlines()

for line in lines:
    cfg, nums = line.split()
    nums = tuple(map(int, nums.split(",")))
    total += count(cfg, nums)

print(total)


# ?.### (1, 1, 3)
# ??.### (1, 1, 3)
# ???.### (1, 1, 3)
# ?.### (1, 3)
# ### (3,)
# 1

# ?.### :: 1
# ??.### :: 1
# ???.### :: 1
# ?.### :: 1
# ### :: 3