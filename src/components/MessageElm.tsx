import { Grid } from "@mui/material";
import Box from "@mui/material/Box";
import { ReactNode } from "react";

interface MessageElmProps {
  text: string;
  avatar: ReactNode;
}

export default function MessageElm({ text, avatar }: MessageElmProps) {
  // convert \n to <br />
  const content = text.split("\n").map((str, index) => (
    <span key={index}>
      {str}
      <br />
    </span>
  ));

  return (
    <>
      <Grid
        container
        sx={{
          width: "768px",
          justifyContent: "flex-start",
          m: "0 auto",
        }}
        spacing={2}
      >
        <Grid
          item
          xs={1}
          sx={{
            display: "flex",
            flexDirection: "row-reverse",
          }}
        >
          {avatar}
        </Grid>
        <Grid item xs={11}>
          <Box
            sx={{
              p: 1,
            }}
          >
            {content}
          </Box>
        </Grid>
      </Grid>
    </>
  );
}
