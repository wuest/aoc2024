module Main where

import Prelude
import Data.List ( transpose, sort )

import qualified Data.Map.Strict as Map

type FreqMap = Map.Map Int Int

freqMap :: FreqMap -> Int -> FreqMap
freqMap acc key = Map.insertWith (+) key 1 acc

freqScore :: FreqMap -> Int -> Int
freqScore map key = key * Map.findWithDefault 0 key map

main :: IO ()
main = do
  input :: [[Int]] <- fmap (fmap read . words) . lines <$> readFile "input.txt"
  let a:b:_ = transpose input
      freq  = foldl freqMap (Map.empty :: FreqMap) b
  print $ sum $ fmap (freqScore freq) a
