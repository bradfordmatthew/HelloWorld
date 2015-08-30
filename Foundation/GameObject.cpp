#include "GameObject.h"

namespace Foundation
{
	short GameObject::SetItemFumbleRate(void)
	{
		short i = 100;

		return i;
	}

	void GameObject::Draw(void)
	{
		//Draw things here...We will get here eventually

	}

	//Returns random game object. Any type, class of given quality.
	GameObject::GameObject(enum eQuality quality, int level) : Item(quality, level)
	{
		m_sFumbleRate = SetItemFumbleRate();
	}


	GameObject::~GameObject()
	{
	}
}
