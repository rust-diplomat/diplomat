import bench from "@stdlib/bench-harness";
import { Opaque } from "../api/Opaque";

bench("Init opaque", () => {
  Opaque.new_();
});
