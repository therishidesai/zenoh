{ self, ... }:
{
  overlays = {
    default = final: _prev: {
      zenoh = self.packages.${final.system};
    };
  };
}
