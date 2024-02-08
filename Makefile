# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: jareste- <jareste-@student.42.fr>          +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2023/10/15 16:37:47 by jareste-          #+#    #+#              #
#    Updated: 2023/11/02 23:07:15 by jareste-         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME = ircserv

#########
RM = rm -f
CC = rustc
CFLAGS = 
#########

#########
FILES = main 

SRC = $(addsuffix .rs, $(FILES))

vpath %.rs srcs
#########

#########
OBJ_DIR = OBJ
OBJ = $(addprefix $(OBJ_DIR)/, $(SRC:.cpp=.o))
DEP = $(addsuffix .d, $(basename $(OBJ)))
#########

#########
$(OBJ_DIR)/%.o: %.rs
	@mkdir -p $(@D)
	@${CC} $(CFLAGS) -MMD -c $< -o $@

all:
	@$(MAKE) $(NAME) --no-print-directory

$(NAME):: $(OBJ)
	@$(CC) $(CFLAGS) $(OBJ) -o $(NAME)
	@echo "EVERYTHING DONE✌️   "⠀⠀⠀⠀⠀⠀⠀


clean:
	@$(RM) $(OBJ) $(DEP) --no-print-directory
	@$(RM) -r $(OBJ_DIR) --no-print-directory
	@echo "OBJECTS REMOVED😭   "

fclean: clean
	@$(RM) $(NAME) --no-print-directory
	@echo "EVERYTHING REMOVED😭   "

re:	fclean all

.PHONY: all clean fclean re

-include $(DEP)
