#include "Item.h"
#include "ConstLits.h"
#include "Enum.h"
#include "TypesDef.h"

namespace Foundation
{
	string* Item::NameItem(eQuality quality)
	{
		string* name = new string;
		switch (quality)
		{
			case Junk:
				if (sizeof(getJunkItemModificationNames[0]) <= 0)
					throw exception("array getJunkItemModificationNames [0] is 0...");

				(*name) += getJunkItemModificationNames[RandomAttrNumber(sizeof(getJunkItemModificationNames) / sizeof(getJunkItemModificationNames[0]))];
				break;
			case Bad:
				if (sizeof(getBadItemModificationNames[0]) <= 0)
					throw exception("array getBadItemModificationNames [0] is 0...");

				(*name) += getBadItemModificationNames[RandomAttrNumber(sizeof(getBadItemModificationNames) / sizeof(getBadItemModificationNames[0]))];
				break;
			case Normal:
				if (sizeof(getNormalItemModificationNames[0]) <= 0)
					throw exception("array getNormalItemModificationNames [0] is 0...");

				(*name) += getNormalItemModificationNames[RandomAttrNumber(sizeof(getNormalItemModificationNames) / sizeof(getNormalItemModificationNames[0]))];
				break;
			case Good:
				if (sizeof(getGoodItemModificationNames[0]) <= 0)
					throw exception("array getGoodItemModificationNames [0] is 0...");

				(*name) += getGoodItemModificationNames[RandomAttrNumber(sizeof(getGoodItemModificationNames) / sizeof(getGoodItemModificationNames[0]))];
				break;
			case Great:
			case Exceptional:
			case Legendary:
				if (sizeof(getBestModificationNames[0]) <= 0)
					throw exception("array getBestModificationNames [0] is 0...");

				(*name) += getBestModificationNames[RandomAttrNumber(sizeof(getBestModificationNames) / sizeof(getBestModificationNames[0]))];
				break;
		}
		return name;
	}

	pAttribute Item::GetPrimaryAttribute(vAttributes* attr)
	{
		pAttribute primary;
		primary.first = NO_ATTRIBUTE_ENUM;

		for (vAttributes::iterator itr = (*attr).begin(); itr != (*attr).end(); ++itr)
		{
			if (primary.first == NO_ATTRIBUTE_ENUM)
				primary = (*itr);
			else if (primary.second < (*itr).second)
				primary = (*itr);
		}

		return primary;
	}

	int m_calls = 20;
	void Item::Seed()
	{
		unsigned int time_ui = static_cast<unsigned int>(time(NULL));
		srand((time_ui)*m_calls);
		++m_calls += 1000;
	}

	eItemBaseDamageType Item::GetBaseDamage()
	{
		int damToAdd = RandomAttrNumber(MAX_ITEM_DAMAGE_TYPE_ENUM);

		return (eItemBaseDamageType)damToAdd;
	}

	pAttribute Item::RandomAttribute(eQuality quality, int level)
	{
		pAttribute att;
		att.first = NO_ATTRIBUTE_ENUM;
		unsigned short attToAdd = RandomAttrNumber(MAX_ATTRIBUTE_ENUM);
		short attrValue = RandomAttrValue(quality, level);

		if (attrValue != 0)
		{
			att.first = (eAttributes)attToAdd;
			att.second = attrValue;
		}

		return att;
	}

	void Item::GetItemAttributes(eQuality quality, int level, vAttributes* attributes)
	{
		int iNumAttributes = RandomAttrNumber(quality);

		for (int i = -level; i <= iNumAttributes; i++)
		{
			pAttribute att = RandomAttribute(quality, level);
			if (att.first != NO_ATTRIBUTE_ENUM)
			{
				if((*attributes).empty() == false)
				{
					if (quality > Normal)
					{
						bool found = false;
						for each (pAttribute var in (*attributes))
						{
							if (var.first == att.first)
								found = true;
						}
						if (found)
						{
							for (vAttributes::iterator itr = (*attributes).begin(); itr != (*attributes).end(); ++itr)
							{
								if (itr->first == att.first)
								{
									itr->second += att.second;
								}
							}
						}
						else
						{
							(*attributes).insert(att);
						}
					}
					else
					{
						(*attributes).insert(att);
					}
				}
				else
				{
					(*attributes).insert(att);
				}
			}
		}
	}

	unsigned short Item::RandomAttrNumber(unsigned short high)
	{
		Seed();
		if (high == 0)
			high = 1;

		unsigned short x;
		x = rand() % high;
		return x;
	}

	short Item::RandomAttrValue(eQuality quality, int level)
	{
		Seed();
		short x;

		short negMod = 1;
		switch (quality)
		{
		case Junk:
		case Bad:
		case Normal:
			negMod = 2;
			break;
		case Good:
			negMod = 4;
			break;
		case Great:
			negMod = 6;
			break;
		case Exceptional:
			negMod = 8;
			break;
		case Legendary:
			negMod = 20;
			break;
		case God:
			negMod = 80;
			break;
		}

		x = (rand() % quality);

		if (x%negMod == 0)
			x *= -1;

		if (x != 0 && x > 0)
			x += level;

		return x;
	}

	void Item::AttributeValueAddition(const vAttributes* attributes, iItemValue* value)
	{
		int attributePosCount = 0;
		int attributeNegCount = 0;
		int totalAttributes = 0;

		int gold = 0;
		int silver = 0;
		int copper = 0;

		for each (pAttribute var in (*attributes))
		{
			totalAttributes += var.second;
			if (var.second > 0)
				++attributePosCount;
			else
				++attributeNegCount;
		}

		if (totalAttributes > 100)
		{
			gold += totalAttributes / 100;
			silver += totalAttributes % 100;
			copper += RandomAttrNumber(100);
		}
		else
		{
			int rnd = RandomAttrNumber(totalAttributes);
			silver += rnd;
			if (totalAttributes > 10)
				copper += RandomAttrNumber(totalAttributes);
		}

		value->gold += gold;
		value->silver += silver;
		value->copper += (copper + totalAttributes*50);
	}

	void Item::QualityValueAddition(eQuality quality, iItemValue* value)
	{
		int randomMultiplyer = RandomAttrNumber((quality*10)+3);

		if (quality == God)
			randomMultiplyer += 1000;
		if (quality == Legendary)
			randomMultiplyer += 50;

		if (randomMultiplyer > 0)
		{
			value->copper *= randomMultiplyer;
			value->silver *= randomMultiplyer;
			value->gold *= randomMultiplyer;
		}
	}

	void Item::GetItemValue(eQuality quality, const vAttributes* attributes, iItemValue* value)
	{
		(*value).gold = 0;
		(*value).silver = 0;
		(*value).copper = 1;

		switch (quality)
		{
		case Junk:
			AttributeValueAddition(attributes, value);
			QualityValueAddition(quality, value);
			break;
		case Bad:
			AttributeValueAddition(attributes, value);
			QualityValueAddition(quality, value);
			break;
		case Normal:
			AttributeValueAddition(attributes, value);
			QualityValueAddition(quality, value);
			break;
		case Good:
			AttributeValueAddition(attributes, value);
			QualityValueAddition(quality, value);
			break;
		case Great:
			AttributeValueAddition(attributes, value);
			QualityValueAddition(quality, value);
			break;
		case Exceptional:
			AttributeValueAddition(attributes, value);
			QualityValueAddition(quality, value);
			break;
		case Legendary:
			AttributeValueAddition(attributes, value);
			QualityValueAddition(quality, value);
			break;
		case God:
			AttributeValueAddition(attributes, value);
			QualityValueAddition(quality, value);
			break;
		}
	}

	void Item::DistributeItemValue(iItemValue* value)
	{
		if (value->copper / 100 >= 1)
		{
			value->silver += value->copper / 100;
			value->copper = value->copper % 100;
		}
		if (value->silver / 100 >= 1)
		{
			value->gold += value->silver / 100;
			value->silver = value->silver % 100;
		}
	}

	//Mostly Diagnostic for dev. Some method of this type at some point will display something like "Cracked Sword" That will be added to strings like "A Cracked Sword lays here."
	void Item::Print()
	{
		if (m_bGod || m_bLegendary)
		{
			std::cout << " Name:" << (*m_sName) + "\r\n Copper:" << m_iValue->copper << " Silver:" << m_iValue->silver << " Gold:" << m_iValue->gold << " Weight:" << m_iWeight << "\r\n Fumble:" << m_sFumbleRate << " Base Damage:" << geteItemDamageType[m_eDamageType] << "\r\n Primary Attribute:" << geteAttributes[m_pPrimaryItemAttribute.first] << " ";

			for (vAttributes::iterator i = (*m_vAttributes).begin(); i != (*m_vAttributes).end(); ++i)
			{
				std::cout << " " << geteAttributes[(*i).first] << " " << (*i).second;
			}

			std::cout << "\r\n-------------------------------------\r\n";
		}
	}

	//TODO: Random boost. 1/1000 or so chance that any item *may have a bumped quality
	//string name, int value, int weight, short fumbleRate -> these should all be determined after random attributes are added
	Item::Item(const eQuality quality, const int level)
	{
		m_vAttributes = new vAttributes();
		m_iValue = new iItemValue();
		m_bHasAttributes = false;
		m_bLegendary = false;
		m_pPrimaryItemAttribute.first = NO_ATTRIBUTE_ENUM;
		eQuality qual = quality;

		if (Legend == RandomAttrNumber(Legend + 1))
		{
			qual = Legendary;
			m_bLegendary = true;
			if (Critical == RandomAttrNumber(Legend) || Legend == RandomAttrNumber(Legend + 1) || RandomAttrNumber(Legend) == level)
			{
				qual = God;
				m_bGod = true;
			}
		}

		GetItemAttributes(qual, level, m_vAttributes);
		m_pPrimaryItemAttribute = GetPrimaryAttribute(m_vAttributes);
		m_eDamageType = GetBaseDamage();
		m_sName = NameItem(quality);
		GetItemValue(qual, m_vAttributes, m_iValue);
		DistributeItemValue(m_iValue);

		if ((*m_vAttributes).begin() != (*m_vAttributes).end())
			m_bHasAttributes = true;
	}

	Item::~Item()
	{
		delete m_vAttributes;
		m_vAttributes = 0;
		delete m_sName;
		m_sName = 0;
		delete m_iValue;
		m_iValue = 0;
	}

}
