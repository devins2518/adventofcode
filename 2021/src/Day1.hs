module Day1 ( day1Main ) where

day1Main :: IO ()
day1Main = do
    content <- readFile "input/day1.txt"
    putStrLn "Day1:"
    print $ countIncreases (map read (lines content))
    print $ countSlidingIncreases (map read (lines content))

countIncreases :: [Int] -> Int
countIncreases [] = 0
countIncreases [_] = 0
countIncreases (x:xs) = fromEnum ( x < head xs ) + countIncreases xs

countSlidingIncreases :: [Int] -> Int
countSlidingIncreases [] = 0
countSlidingIncreases xs = fromEnum (sumOfThree xs < sumOfThree (tail xs)) + countSlidingIncreases (tail xs)

sumOfThree :: [Int] -> Int
sumOfThree [x] = x
sumOfThree [x, y] = x + y
sumOfThree xs = sum (take 3 xs)
