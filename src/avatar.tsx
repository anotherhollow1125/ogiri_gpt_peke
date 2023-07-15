import { Message } from "@/structs";
import { green, orange, blue, purple } from "@mui/material/colors";
import AssistantIcon from "@mui/icons-material/Assistant";
import PersonIcon from "@mui/icons-material/Person";
import { Avatar } from "@mui/material";

export function get_avatar(message: Message) {
  switch (message.character) {
    case "user":
      return (
        <Avatar sx={{ bgcolor: green[500] }} variant="rounded">
          <PersonIcon />
        </Avatar>
      );
    case "melchior":
      return (
        <Avatar sx={{ bgcolor: orange[500] }} variant="rounded">
          <AssistantIcon />
        </Avatar>
      );
    case "balthasar":
      return (
        <Avatar sx={{ bgcolor: blue[500] }} variant="rounded">
          <AssistantIcon />
        </Avatar>
      );
    case "casper":
      return (
        <Avatar sx={{ bgcolor: purple[500] }} variant="rounded">
          <AssistantIcon />
        </Avatar>
      );
    default:
      return (
        <Avatar sx={{ bgcolor: green[500] }} variant="rounded">
          <AssistantIcon />
        </Avatar>
      );
  }
}
