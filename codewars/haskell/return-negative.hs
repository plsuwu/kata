makeNegative :: (Num a, Ord a) => a -> a
makeNegative n = inverted n
  where
    inverted n
        | n < 0 = n
        | otherwise = n * (-1)
