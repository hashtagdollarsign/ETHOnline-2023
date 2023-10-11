import * as React from 'react';
import { styled } from '@mui/material/styles';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';
import MenuDrawer from '../Drawer/MenuDrawer';
import Lcd from '../Lcd/Lcd';



const Item = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.mode === 'dark' ? '#1A2027' : '#fff',
  ...theme.typography.body2,
  padding: theme.spacing(0),
  textAlign: 'center',
  color: theme.palette.text.secondary,
}));

export default function Wrapper() {
  return (
    <Box sx={{ width: 1 }}>
      <Box display="grid" gridTemplateColumns="repeat(12, 1fr)" gap={2}>
        <Box gridColumn="span 12">
          <Item sx={{ height: 600  }} ><Lcd/> </Item>
        </Box>
         <Box gridColumn="span 12">
          <Item sx={{ height: 50 }} > 
          <MenuDrawer />
          </Item>
        </Box>
        <Box gridColumn="span 6">
          <Item sx={{ height: 200 }}>xs=4</Item>
        </Box>
        <Box gridColumn="span 6">
          <Item sx={{ height: 200 }}>xs=4</Item>
        </Box>
       
      </Box>
    </Box>
  );
}