const UE = require("ue");
const { GameplayTagUtils } = require("../Core/Utils/GameplayTagUtils.js");
const {
  SpecialEnergyBarBase,
} = require("../Game/Module/BattleUi/Views/SpecialEnergy/SpecialEnergyBarBase.js");
const {
  SpecialEnergyBarSlot,
} = require("../Game/Module/BattleUi/Views/SpecialEnergy/SpecialEnergyBarSlot.js");
const {
  RoleSpecialEnergyBar,
} = require("../Game/Module/BattleUi/Views/SpecialEnergy/RoleSpecialEnergyBar.js");
const ModelManager_1 = require("../Game/Manager/ModelManager.js");

const FORM_LIGHT_CONFIG_ID = 151201;
const LILI_EXTRA_TYPE_ID = 151202;
const FormLightTagId = GameplayTagUtils.GetTagIdByName(
  "角色.R2T1LiliMd10011.逻辑标识.美梦标识",
);
const StateEnhancedTagId = GameplayTagUtils.GetTagIdByName(
  "角色.R2T1LiliMd10011.技能标识.莉莉重击引导标识",
);

const EForm = { Light: 0, Dark: 1, Max: 2 };
const EState = { Normal: 0, Active: 1, Enhanced: 2, Max: 3 };

class SpecialEnergyBarLili extends SpecialEnergyBarBase {
  constructor(...args) {
    super(...args);
    this.ConfigLight = undefined;
    this.ConfigDark = undefined;
    this.BarItemLight = undefined;
    this.BarItemDark = undefined;
    this.CurForm = EForm.Max;
    this.CurState = EState.Max;
    this.StateRefreshTimer = null;

    this.OnFormTagChanged = (tagId, tagExist) => {
      this.SetForm(tagExist ? EForm.Light : EForm.Dark);
    };

    this.OnStateEnhancedTagChanged = (tagId, tagExist) => {
      this.RequestRefreshState();
    };
  }

  OnRegisterComponent() {
    for (let i = 0; i <= 9; i++) {
      this.ComponentRegisterInfos.push([i, UE.UIItem]);
    }
  }

  OnInitData() {
    this.ConfigLight =
      ModelManager_1.ModelManager.BattleUiModel.SpecialEnergyBarData.GetSpecialEnergyBarInfo(
        FORM_LIGHT_CONFIG_ID,
      );
  }

  async OnBeforeStartAsync() {
    await Promise.all([this.InitBarItemLight(), this.InitBarItemDark()]);
  }

  async InitBarItemLight() {
    if (this.ConfigLight) {
      this.BarItemLight = new SpecialEnergyBarSlot();
      this.BarItemLight.InitData(this.RoleData, this.ConfigLight);
      await this.BarItemLight.InitByActorAsync(this.GetItem(1).GetOwner());
    }
  }

  async InitBarItemDark() {
    this.BarItemDark = new SpecialEnergyBarSlot();
    this.BarItemDark.InitData(this.RoleData, this.Config);
    await this.BarItemDark.InitByActorAsync(this.GetItem(4).GetOwner());
  }

  OnStart() {
    this.InitTweenAnim(2);
    this.InitTweenAnim(5);
    this.InitTweenAnim(6);
    this.InitTweenAnim(7);
    this.InitTweenAnim(8);
    this.InitTweenAnim(9);
    this.RefreshForm(true);
  }

  AddEvents() {
    super.AddEvents();
    this.ListenForTagAddOrRemoveChanged(FormLightTagId, this.OnFormTagChanged);
    this.ListenForTagAddOrRemoveChanged(
      StateEnhancedTagId,
      this.OnStateEnhancedTagChanged,
    );
  }

  OnBeforeDestroy() {
    this.CancelStateRefresh();
    super.OnBeforeDestroy();
  }

  OnBarPercentChanged() {
    super.OnBarPercentChanged?.();
    this.RequestRefreshState();
  }

  RefreshForm(isForced = false) {
    const flag = this.TagComponent?.HasTag(FormLightTagId) ?? false;
    this.SetForm(flag ? EForm.Light : EForm.Dark, isForced);
  }

  SetForm(form, isForced = false) {
    if (this.CurForm === form && !isForced) return;

    this.CurForm = form;

    if (this.CurForm === EForm.Light) {
      this.StopTweenAnim(5);
      this.PlayTweenAnim(2);
    } else {
      this.StopTweenAnim(2);
      this.PlayTweenAnim(5);
    }

    this.CancelStateRefresh();
    this.RefreshStateImmediately(true);
  }

  RequestRefreshState() {
    if (this.StateRefreshTimer == null) {
      this.StateRefreshTimer = Promise.resolve().then(() => {
        this.StateRefreshTimer = null;
        this.RefreshStateImmediately();
      });
    }
  }

  CancelStateRefresh() {
    this.StateRefreshTimer = null;
  }

  RefreshStateImmediately(isForced = false) {
    const isEnergyFull = this.PercentMachine.GetCurPercent() >= 1.0;
    const enhancedTagExist =
      this.TagComponent?.HasTag(StateEnhancedTagId) ?? false;
    this.SetState(this.CalcCurState(isEnergyFull, enhancedTagExist), isForced);
  }

  CalcCurState(isEnergyFull, enhancedTagExist) {
    if (enhancedTagExist) return EState.Enhanced;
    if (!isEnergyFull) return EState.Normal;
    return EState.Active;
  }

  SetState(state, isForced = false) {
    if (this.CurState === state && !isForced) return;

    const prevState = this.CurState;
    this.CurState = state;

    const wasActive =
      prevState === EState.Active || prevState === EState.Enhanced;
    const isActive =
      this.CurState === EState.Active || this.CurState === EState.Enhanced;
    if (wasActive !== isActive || isForced) {
      if (isActive) {
        this.StopTweenAnim(7);
        this.PlayTweenAnim(6);
      } else {
        this.StopTweenAnim(6);
        this.PlayTweenAnim(7);
      }
    }

    const wasEnhanced = prevState === EState.Enhanced;
    const isEnhanced = this.CurState === EState.Enhanced;
    if (wasEnhanced !== isEnhanced || isForced) {
      if (isEnhanced) {
        this.StopTweenAnim(9);
        this.PlayTweenAnim(8);
      } else {
        this.StopTweenAnim(8);
        this.PlayTweenAnim(9);
      }
    }
  }

  Tick(delta) {
    super.Tick(delta);
    this.BarItemLight?.Tick(delta);
    this.BarItemDark?.Tick(delta);
  }
}

const originalCdt = RoleSpecialEnergyBar.prototype.cdt;
RoleSpecialEnergyBar.prototype.cdt = async function (e, r, a, i) {
  if (i.ExtraType === LILI_EXTRA_TYPE_ID) {
    const instance = new SpecialEnergyBarLili();
    instance.InitData(r, i);
    this.hdt.set(a, instance);
    await instance.InitByPathAsync(e, i.PrefabPath);
    return;
  }
  return originalCdt.apply(this, arguments);
};
