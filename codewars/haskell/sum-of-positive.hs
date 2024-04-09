positiveSum :: [Int] -> Int
positiveSum xs = sum (filter (> 0) xs)
