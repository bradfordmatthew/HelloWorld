
namespace Foundation
{
	enum eLuckyRolls
	{
		Legend = 999,
		Critical = 111
	};

	enum eItemBaseDamageType
	{
		Blunt,
		Edge,
		Pierce,
		Arcane,
		MAX_ITEM_DAMAGE_TYPE_ENUM
	};

	enum eAttributes
	{
		Intelligence,
		Strength,
		Dexterity,
		Wisdom,
		Luck,
		MAX_ATTRIBUTE_ENUM,
		NO_ATTRIBUTE_ENUM
	};

	enum eQuality
	{
		Junk= 0,
		Bad= 1,
		Normal= 2,
		Good= 7,
		Great= 12,
		Exceptional= 20,
		Legendary= 30,
		God=1000
	};
}