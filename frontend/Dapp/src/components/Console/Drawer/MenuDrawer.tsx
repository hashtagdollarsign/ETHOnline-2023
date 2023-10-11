import * as React from 'react';
import Box from '@mui/material/Box';
import Drawer from '@mui/material/Drawer';
import Button from '@mui/material/Button';
import List from '@mui/material/List';
import Divider from '@mui/material/Divider';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import InboxIcon from '@mui/icons-material/MoveToInbox';
import MailIcon from '@mui/icons-material/Mail';
import { ConnectButton } from '@rainbow-me/rainbowkit';


import MenuSurface from '../MenuSurface/MenuSurface';
type Anchor = 'left';

export default function MenuDrawer() {
  const [state, setState] = React.useState({

    left: false,
  });

  const toggleDrawer =
    (anchor: Anchor, open: boolean) =>
    (event: React.KeyboardEvent | React.MouseEvent) => {
      if (
        event.type === 'keydown' &&
        ((event as React.KeyboardEvent).key === 'Tab' ||
          (event as React.KeyboardEvent).key === 'Shift')
      ) {
        return;
      }

      setState({ ...state, [anchor]: open });
    };

  const list = (anchor: Anchor) => (
    <><MailIcon onClick={toggleDrawer(anchor, false)}>

      </MailIcon>

       <ConnectButton />
      <Box
          sx={{ width: 'auto' }}
          role="presentation"
          onClick={toggleDrawer(anchor, true)}
          onKeyDown={toggleDrawer(anchor, false)}
      >
           
                  <MenuSurface />
              
              <Divider />
              <List>
                  {['Friends', 'LeaderBoard'].map((text, index) => (
                      <ListItem key={text} disablePadding>
                          <ListItemButton>
                              <ListItemIcon>
                                  {index % 2 === 0 ? <InboxIcon /> : <MailIcon />}
                              </ListItemIcon>
                              <ListItemText primary={text} />
                          </ListItemButton>
                      </ListItem>
                  ))}
              </List>
          </Box></>
  );

  return (
    <div>
     
        <React.Fragment key={'left'}>
          <Button onClick={toggleDrawer('left', true)}>MENU</Button>
          <Drawer
            anchor={'left'}
            open={state['left']}
            onClose={toggleDrawer('left', false)}
          >
            {list('left')}
          </Drawer>
        </React.Fragment>
     
    </div>
  );
}