const path = require("path");
const fs = require("fs");
const k = require("@metaplex-foundation/kinobi");
const crypto = require("crypto");

const idlsDir = path.join(__dirname, "idls");

function generateSolautoSDK() {
  const kinobi = k.createFromIdls([path.join(idlsDir, "solauto.json")]);

  kinobi.update(
    k.updateProgramsVisitor({
      idl: { name: "solauto" },
    })
  );

  kinobi.accept(
    new k.renderRustVisitor(
      path.join(__dirname, "programs", "solauto-sdk", "src", "generated")
    )
  );

  kinobi.accept(
    new k.renderJavaScriptVisitor(
      path.join(__dirname, "typescript", "solauto-sdk", "src", "generated")
    )
  );
}

function generateSDKForAnchorIDL(sdkDirName, idlFilename, programId) {
  const idlFilePath = path.join(idlsDir, idlFilename);

  const rawData = fs.readFileSync(idlFilePath, "utf8");
  let data = JSON.parse(rawData);
  data.metadata = {
    origin: "anchor",
    address: programId,
  };
  fs.writeFileSync(idlFilePath, JSON.stringify(data, null, 2), "utf8");

  const kinobi = k.createFromIdls([idlFilePath]);

  kinobi.update(
    k.updateProgramsVisitor({
      idl: { sdkDirName },
    })
  );

  kinobi.accept(
    new k.renderRustVisitor(
      path.join(__dirname, "programs", sdkDirName, "src", "generated")
    )
  );
}

generateSolautoSDK();
// generateSDKForAnchorIDL(
//   "marginfi-sdk",
//   "marginfi.json",
//   "MFv2hWf31Z9kbCa1snEPYctwafyhdvnV7FZnsebVacA"
// );
