module Main where

import Prelude
import Data.List ( transpose, sort )

main :: IO ()
main = do
  input :: [[Int]] <- fmap (fmap read . words) . lines <$> readFile "input.txt"
  let a:b:_ = transpose input
  print $ sum $ abs <$> zipWith (-) (sort a) (sort b)
