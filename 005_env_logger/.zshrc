# If you come from bash you might have to change your $PATH.
export PATH=$HOME/bin:/usr/local/bin:$PATH

# Path to your oh-my-zsh installation.
export ZSH="/Users/liwei/.oh-my-zsh"

# Set name of the theme to load --- if set to "random", it will
# load a random theme each time oh-my-zsh is loaded, in which case,
# to know which specific one was loaded, run: echo $RANDOM_THEME
# See https://github.com/ohmyzsh/ohmyzsh/wiki/Themes
ZSH_THEME="robbyrussell"

# Set list of themes to pick from when loading at random
# Setting this variable when ZSH_THEME=random will cause zsh to load
# a theme from this variable instead of looking in $ZSH/themes/
# If set to an empty array, this variable will have no effect.
# ZSH_THEME_RANDOM_CANDIDATES=( "robbyrussell" "agnoster" )

# Uncomment the following line to use case-sensitive completion.
# CASE_SENSITIVE="true"

# Uncomment the following line to use hyphen-insensitive completion.
# Case-sensitive completion must be off. _ and - will be interchangeable.
# HYPHEN_INSENSITIVE="true"

# Uncomment the following line to disable bi-weekly auto-update checks.
# DISABLE_AUTO_UPDATE="true"

# Uncomment the following line to automatically update without prompting.
# DISABLE_UPDATE_PROMPT="true"

# Uncomment the following line to change how often to auto-update (in days).
# export UPDATE_ZSH_DAYS=13

# Uncomment the following line if pasting URLs and other text is messed up.
# DISABLE_MAGIC_FUNCTIONS="true"

# Uncomment the following line to disable colors in ls.
# DISABLE_LS_COLORS="true"

# Uncomment the following line to disable auto-setting terminal title.
# DISABLE_AUTO_TITLE="true"

# Uncomment the following line to enable command auto-correction.
# ENABLE_CORRECTION="true"

# Uncomment the following line to display red dots whilst waiting for completion.
# COMPLETION_WAITING_DOTS="true"

# Uncomment the following line if you want to disable marking untracked files
# under VCS as dirty. This makes repository status check for large repositories
# much, much faster.
# DISABLE_UNTRACKED_FILES_DIRTY="true"

# Uncomment the following line if you want to change the command execution time
# stamp shown in the history command output.
# You can set one of the optional three formats:
# "mm/dd/yyyy"|"dd.mm.yyyy"|"yyyy-mm-dd"
# or set a custom format using the strftime function format specifications,
# see 'man strftime' for details.
# HIST_STAMPS="mm/dd/yyyy"

# Would you like to use another custom folder than $ZSH/custom?
# ZSH_CUSTOM=/path/to/new-custom-folder

# Which plugins would you like to load?
# Standard plugins can be found in $ZSH/plugins/
# Custom plugins may be added to $ZSH_CUSTOM/plugins/
# Example format: plugins=(rails git textmate ruby lighthouse)
# Add wisely, as too many plugins slow down shell startup.
#plugins=(git)

source $ZSH/oh-my-zsh.sh

# User configuration

# export MANPATH="/usr/local/man:$MANPATH"

# You may need to manually set your language environment
# export LANG=en_US.UTF-8

# Preferred editor for local and remote sessions
# if [[ -n $SSH_CONNECTION ]]; then
#   export EDITOR='vim'
# else
#   export EDITOR='mvim'
# fi

# Compilation flags
# export ARCHFLAGS="-arch x86_64"

# Set personal aliases, overriding those provided by oh-my-zsh libs,
# plugins, and themes. Aliases can be placed here, though oh-my-zsh
# users are encouraged to define aliases within the ZSH_CUSTOM folder.
# For a full list of active aliases, run `alias`.
#
# Example aliases
# alias zshconfig="mate ~/.zshrc"
# alias ohmyzsh="mate ~/.oh-my-zsh"

# added 
# Path to your oh-my-zsh installation.
export ZSH="/Users/liwei/.oh-my-zsh";
export PATH=/opt/local/bin:/opt/local/sbin:/Applications/xampp/xamppfiles/bin:$PATH


ZSH_THEME="powerlevel10k/powerlevel10k"
# POWERLEVEL9K_MODE="awesome-patched"

# command line 左邊想顯示的內容
POWERLEVEL9K_LEFT_PROMPT_ELEMENTS=(dir dir_writable vcs virtualenv)# <= left prompt 設了 "dir"
# command line 右邊想顯示的內容
# POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS=(status time) # <= right prompt 設了 "time"

POWERLEVEL9K_MODE='nerdfont-complete'

source /usr/local/share/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh

# zsh 支持的插件

# zsh-autosuggestions 自动补全设置
#ZSH_AUTOSUGGEST_HIGHLIGHT_STYLE="fg=#ff00ff,bg=cyan,bold,underline"
#
# Prompts
POWERLEVEL9K_LEFT_SEGMENT_SEPARATOR='\uE0C0'
#POWERLEVEL9K_LEFT_SUBSEGMENT_SEPARATOR='\uE0C0'
POWERLEVEL9K_RIGHT_SEGMENT_SEPARATOR='\uE0C2'
#POWERLEVEL9K_RIGHT_SUBSEGMENT_SEPARATOR='\uE0C2'
#POWERLEVEL9K_LEFT_PROMPT_ELEMENTS=(dir dir_writable vcs virtualenv)
POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS=(status time background_jobs ssh)
#POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS=(status time background_jobs command_execution_time ip)
POWERLEVEL9K_SHORTEN_DIR_LENGTH=1
POWERLEVEL9K_SHORTEN_DELIMITER=..
#POWERLEVEL9K_PROMPT_ON_NEWLINE=true
#POWERLEVEL9K_MULTILINE_FIRST_PROMPT_PREFIX=$'\n'
POWERLEVEL9K_MULTILINE_LAST_PROMPT_PREFIX="\uF460%F{073}\uF460%F{109}\uF460%f "
POWERLEVEL9K_MODE='nerdfont-complete'
POWERLEVEL9K_DIR_SHORTENED_FOREGROUND=238
POWERLEVEL9K_TIME_BACKGROUND=255
ZSH_THEME="powerlevel10k/powerlevel10k"
export UPDATE_ZSH_DAYS=13
plugins=(
    z
    bundler
    dotenv
    rake
    rbenv
    ruby
    git
    colored-man-pages
    colorize
    github
    brew
    macos
    docker
    docker-compose
    autojump
    zsh-autosuggestions
    zsh-syntax-highlighting
    autopep8
    python
)
source $ZSH/oh-my-zsh.sh
alias zshconfig="vim ~/.zshrc"
alias vimconfig="vim ~/.vimrc"
#alias ansibleconfig="vim ~/.ansible/ansible.cfg"
alias grep='grep --color=auto'



# pipenv 自动补齐
eval "$(pipenv --completion)"

# golang setting
# gopath表示代码目录
export GOPATH=$HOME/go

# gobin 表示我们开发程序编译后二进制命令的安装目录
export GOBIN=$GOPATH/bin
export GOROOT=/usr/local/go

export PATH=$GOROOT/bin:$GOBIN:$PATH

# Enable the go modules feature
export GO111MODULE=on
# Set the GOPROXY environment variable
export GOPROXY=https://goproxy.io
#export GOPROXY=https://goproxy.cn,direct


# filecoin
#export IPFS_GATEWAY=https://proof-parameters.s3.cn-south-1.jdcloud-oss.com/ipfs/
#export FIL_PROOFS_PARAMETER_CACHE=/Users/liwei/coding/devnet/proofs
#export TMPDIR=/Users/liwei/coding/devnet/tmp
#export RUST_LOG=Debug
# 编译底层库
#export FFI_BUILD_FROM_SOURCE=1
# 小扇区支持
#export FIL_USE_SMALL_SECTORS=true

# >>> conda initialize >>>
# !! Contents within this block are managed by 'conda init' !!
__conda_setup="$('/Users/liwei/opt/anaconda3/bin/conda' 'shell.zsh' 'hook' 2> /dev/null)"
if [ $? -eq 0 ]; then
    eval "$__conda_setup"
else
    if [ -f "/Users/liwei/opt/anaconda3/etc/profile.d/conda.sh" ]; then
        . "/Users/liwei/opt/anaconda3/etc/profile.d/conda.sh"
    else
        export PATH="/Users/liwei/opt/anaconda3/bin:$PATH"
    fi
fi
unset __conda_setup
# <<< conda initialize <<<


export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion

source /usr/local/share/zsh-history-substring-search/zsh-history-substring-search.zsh
bindkey -M emacs '^P' history-substring-search-up
bindkey -M emacs '^N' history-substring-search-down
