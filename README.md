# [elite](https://github.com/ferhatgec/elite)toscala
## [elite](https://github.com/ferhatgec/elite) -> scala converter

### input:
```rs
required_version is 0.1

set ProjectName as "elitetoscala"
set HOME        as env "HOME"


for argument "install" [
    use exec "cargo install --path ."

    for exists "{HOME}.cargo/bin/{ProjectName}" [
        println "{ProjectName} installed to {HOME}.cargo/bin/{ProjectName}."
    ]

    use signal "exit"
]
```

### output
```scala
import System.getProperty
import scala.language.postfixOps
import java.nio.file.{ Paths, Files }
import scala.sys.process._

object elite {
 def main(args: Array[String]) = {
  if("0.1" != "0.1")
  {
    println("elite: Required higher version")
    sys.exit(1);
  }
  val ProjectName = "elitetoscala"
  val HOME = "/home/gech"
  if(args.length >= 2 && args(1) == "install")
  {
   Process("cargo install --path .")!
   val path = Files.exists(Paths.get("/home/gech/.cargo/bin/elitetoscala"))
   if(path)   {
    println("elitetoscala installed to /home/gech/.cargo/bin/elitetoscala.\n")
   }
   sys.exit(1)
  }
 }
}
```

### elitetoscala licensed under the terms of MIT License
