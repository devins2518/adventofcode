module Day1 ( day1Main ) where

day1Main :: IO ()
day1Main = do
    content <- readFile "input/day1.txt"
    print $ countIncreases (map read ( lines content ))
    print $ countSlidingIncreases (map read ( lines content ))

countIncreases :: [Int] -> Int
countIncreases [] = 0
countIncreases [_] = 0
countIncreases (x:xs) = fromEnum ( x < head xs ) + countIncreases xs

countSlidingIncreases :: [Int] -> Int
countSlidingIncreases xs = sum (windowOfThree xs) + sum (countSlidingIncreases (tail xs))

windowOfThree :: [Int] -> [Int]
windowOfThree [] = []
windowOfThree [x] = [x, 0, 0]
windowOfThree [x, y] = [x, y, 0]
windowOfThree (x:xs) = [x, head xs, head $ tail xs]
