import Control.Concurrent (threadDelay)
import System.IO (hFlush, stdout)

main :: IO ()
main = do
  let chars = ['-', '\\', '|', '/']
  let loop i = do
        putStr $ [chars !! (i `mod` length chars)] ++ "\r"
        hFlush stdout -- flush output buffer
        threadDelay 200000
        loop (i + 1)
  loop 0
