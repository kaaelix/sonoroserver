setTimeout(() => {
  const UiManager_1 = require("../Ui/UiManager");
  const UE = require("ue");
  const { UidView } = require("../Game/Module/UidShow/UidView.js");
  UidView.prototype.OnStart = function () {
    const UiText = this.GetText(0);
    var pos = UiText.RelativeLocation;
    pos.X = -888;
    pos.Y = 37;
    pos.Z = 0;
    const rawUid = "{PLR_UID}";
    const maskedUid = rawUid.length > 3 ? "***" + rawUid.slice(-3) : "***";
    UiText.SetText("UID: " + maskedUid);
    UiText.SetColor(UE.Color.FromHex("#f6c575"));
    UiText.SetAlpha(0.75);
    UiText.SetFontSize(25);
    UiText.SetParagraphHorizontalAlignment(2);

    setTimeout(() => {
      UiText.SetUIRelativeLocation(pos);
    }, 500);
  };
}, 0);
