import { useState } from "react";
import Paper from "@mui/material/Paper";
import { TextField } from "@mui/material";
import IconButton from "@mui/material/IconButton";
import SendIcon from "@mui/icons-material/Send";

interface PromptFieldElmProps {
  send: (prompt: string) => Promise<void>;
}

export default function PromptFieldElm({ send }: PromptFieldElmProps) {
  const [prompt, setPrompt] = useState("");

  const sendPrompt = async () => {
    setPrompt("");
    await send(prompt);
  };

  return (
    <Paper
      component="form"
      sx={{
        p: "0px 6px 0px 0px",
        m: "0 auto",
        display: "flex",
        alignItems: "center",
        width: 670,
        borderRadius: "0.75rem",
      }}
    >
      <TextField
        multiline
        sx={{
          px: 1,
          flex: 1,
          "& .MuiOutlinedInput-notchedOutline": {
            border: "none",
          },
        }}
        placeholder={"お題"}
        value={prompt}
        onChange={(e) => {
          setPrompt(e.target.value);
        }}
        onKeyDown={(e: React.KeyboardEvent<HTMLDivElement>) => {
          if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            sendPrompt();
          }
        }}
      />
      <IconButton
        color="primary"
        sx={{ p: "10px" }}
        aria-label="send"
        onClick={(_e) => {
          sendPrompt();
        }}
      >
        <SendIcon />
      </IconButton>
    </Paper>
  );
}
