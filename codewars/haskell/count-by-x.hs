import Data.List
import System.IO

countBy :: Integer -> Int -> [Integer]
countBy x n = take n [x * i | i <- [1 ..]]
