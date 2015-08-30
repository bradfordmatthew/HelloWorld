#include <string>
#include "Enum.h"

namespace Foundation
{
	class Item
	{
	public:
		Item(eQuality quality, int level);
		~Item();
		void Print();
		virtual void Draw(void) = 0;


		//member variables
		const string* m_sName;
		struct iItemValue { unsigned long long gold; unsigned int silver; unsigned int copper; }*m_iValue;
		int m_iWeight;
		short m_sFumbleRate;
		enum eItemBaseDamageType m_eDamageType;
		vAttributes* m_vAttributes;
		pAttribute m_pPrimaryItemAttribute;
		bool m_bHasAttributes;
		bool m_bLegendary;
		bool m_bGod;
	protected:
		//Inherited classes will determine fumble rate. Weapons have a lower rate than objects like tables.
		virtual short SetItemFumbleRate(void)=0;
	private:
		//Produces the values of attributes. Higher quality has less of a chance of producing negative values.
		//Values for the attribute generated in RandomAttrNumber. May return negative.
		static short RandomAttrValue(enum eQuality quality, int level);
		//Produces the values used to determin random attributes. High is the number of attributes in the attributes enum.
		//A random number within the range of the attribute enum. Used to determine what attribute will be added. Always returns positive.
		static unsigned short RandomAttrNumber(unsigned short high);
		//Gets the attributes collection on random items. Higher quality produces more attributes.
		static void GetItemAttributes(enum eQuality quality, int level, vAttributes* attributes);
		//Gets an attribute. Higher quality produces a greater range of possible attribute bonus.
		static pAttribute RandomAttribute(enum eQuality quality, int level);
		//Gets base damage for the item. Bias towards blunt as most items are of that variety. Ie a barrel is certainly not a piercing weapon.
		//Base damage of an item. Will be used to determine possible item sub-classes ie: weapon, armor, or it could just stay item.
		static eItemBaseDamageType GetBaseDamage();
		//Sets the seed. Call before Random.
		static void Seed();
		//Uses attributes and damage type to determine name
		static string* NameItem(eQuality quality);
		//Gets the sub-type of item
		static void GenerateItemType();
		//Gets the Primary Attribute (highest value)
		static pAttribute GetPrimaryAttribute(vAttributes* attributes);
		//Gets Items value
		static void GetItemValue(eQuality quality, const vAttributes* attributes, iItemValue* value);
		//Gets a multiplier for value based on positive and negative item attributes
		static void AttributeValueAddition(const vAttributes* attributes, iItemValue* value);
		static void QualityValueAddition(enum eQuality quality, iItemValue* value);
		//Use this method to re-distribute gold copper and silver to max each.
		static void DistributeItemValue(iItemValue* value);

	};




}
